use super::*;

mod traits;

/// Represents a Tailwind color, which can be a themed color, a keyword, or an arbitrary value.
#[derive(Clone, Debug)]
pub enum TailwindColor {
    RGB(Srgb),
    Themed{name: String, weight: u32, alpha: Option<f32>},
    Keyword { name: String, alpha: Option<f32> },
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RGB(c) => write!(
                f,
                "[#{:02X?}{:02X?}{:02X?}{:02X?}]",
                (255.0 * c.red) as u8,
                (255.0 * c.green) as u8,
                (255.0 * c.blue) as u8,
                (255.0 * c.alpha) as u8
            ),
            Self::Themed { name, weight, alpha } => {
                write!(f, "{}-{}", name, weight)?;
                if let Some(a) = alpha {
                    // Alpha is 0.0-1.0, so convert to percentage for the class name
                    write!(f, "/{}", *a * 100.0)?;
                }
                Ok(())
            }
            Self::Arbitrary(a) => a.write(f),
            Self::Keyword { name, alpha } => {
                match name.as_str() {
                    "transparent" => write!(f, "transparent")?,
                    "current" => write!(f, "current")?,
                    _ => write!(f, "{}", name)?,
                }
                if let Some(a) = alpha {
                    write!(f, "/{}", *a * 100.0)?;
                }
                Ok(())
            }
        }
    }
}

#[allow(non_upper_case_globals)]
impl TailwindColor {
    /// `black`
    pub const Black: Self = Self::RGB(Srgb { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    /// `white`
    pub const White: Self = Self::RGB(Srgb { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    
    /// Parses a color pattern, now with support for opacity modifiers like `/75`.
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        // 1. Separate the main color pattern from the opacity modifier.
        let (main_pattern, opacity_modifier) = split_modifier(pattern);

        // 2. Parse the base color from the main pattern.
        let mut color = match main_pattern.as_slice() {
            ["none"] | ["transparent"] => Self::from("transparent"),
            ["black"] => Self::Black,
            ["white"] => Self::White,
            [s @ ("current" | "inherit" | "initial" | "unset")] => Self::from(*s),

            [] => Self::parse_arbitrary(arbitrary)?,
            [name, weight] => Self::parse_themed(name, weight)?,

            [name] => Self::Keyword {
                name: name.to_string(),
                alpha: None, // The alpha logic handles this later
            },

            _ => return syntax_error!("Unknown color pattern: {}", main_pattern.join("-")),
        };

        // 3. If an opacity modifier exists, parse and apply it.
        if let Some(modifier_str) = opacity_modifier {
            let alpha = modifier_str.parse::<f32>()? / 100.0;
            color.set_alpha(alpha);
        }

        Ok(color)
    }

    // Helper method to apply the alpha value.
    fn set_alpha(&mut self, alpha: f32) {
        match self {
            TailwindColor::RGB(srgb) => srgb.alpha = alpha,
            TailwindColor::Themed { alpha: a, .. } => *a = Some(alpha),
            TailwindColor::Keyword { alpha: a, .. } => *a = Some(alpha),
            TailwindColor::Arbitrary(_) => {
                // Applying alpha to an already-arbitrary value is complex
                // and often not needed, but could be implemented here if required.
            }
        }
    }

    #[inline]
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<TailwindColor> {
        Ok(Self::RGB(arbitrary.as_color()?))
    }

    ///
    #[inline]
    pub fn parse_themed(name: &str, weight: &str) -> Result<TailwindColor> {
        let name = name.to_string();
        let weight = TailwindArbitrary::from(weight).as_integer()? as u32;
        // Initialize with no alpha; it will be added by `parse` if a modifier exists.
        Ok(Self::Themed { name, weight, alpha: None })
    }

    /// get class of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    /// get properties of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_properties(&self, ctx: &TailwindBuilder) -> String {
        match self {
            Self::RGB(c) => format!("rgba({}, {}, {}, {})", 255.0 * c.red, 255.0 * c.green, 255.0 * c.blue, c.alpha),
            Self::Arbitrary(a) => a.get_properties(),
            Self::Keyword { name, alpha } => {
                // First, check for special CSS keywords that should not be looked up in the palette.
                match name.as_str() {
                    "transparent" => return "transparent".to_string(),
                    "current" => return "currentColor".to_string(),
                    _ => (), // Not a special keyword, proceed to palette lookup.
                }

                // Second, try to find the keyword in the palette system.
                match ctx.palettes.try_get_keyword_color(name) {
                    Ok(color_ref) => {
                        // We got a reference, so we copy it to a mutable variable to work with.
                        let mut c = *color_ref;
                        // If an alpha modifier exists on the keyword (e.g., from `primary/50`), apply it.
                        if let Some(a) = alpha {
                            c.alpha = *a;
                        }
                        format!("rgba({}, {}, {}, {})", 255.0 * c.red, 255.0 * c.green, 255.0 * c.blue, c.alpha)
                    }
                    // Fallback if the keyword is not found in the palette.
                    Err(_) => "currentColor".to_string(),
                }
            }
            Self::Themed { name, weight, alpha } => {
                match ctx.palettes.try_get_color(name, *weight) {
                    Ok(mut c) => {
                        if let Some(a) = alpha {
                            c.alpha = *a;
                        }
                        format!("rgba({}, {}, {}, {})", 255.0 * c.red, 255.0 * c.green, 255.0 * c.blue, c.alpha)
                    },
                    Err(_) => "currentColor".to_string(),
                }
            },
        }
    }
}


/// Splits a Tailwind class pattern into its main part and an optional modifier.
///
/// e.g., `["red", "500/75"]` -> `(vec!["red", "500"], Some("75"))`
/// e.g., `["red", "500"]`    -> `(vec!["red", "500"], None)`
fn split_modifier<'a>(pattern: &'a [&'a str]) -> (Vec<&'a str>, Option<&'a str>) {
    if let Some((last, initial_parts)) = pattern.split_last() {
        if let Some((main, modifier)) = last.rsplit_once('/') {
            let mut main_pattern = initial_parts.to_vec();
            main_pattern.push(main);
            return (main_pattern, Some(modifier));
        }
    }
    // If no "/" is found, return the original pattern and no modifier.
    (pattern.to_vec(), None)
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_custom_keyword_parsing_and_resolution() {
        // ARRANGE: Set up the context with a custom keyword.
        // 1. Create a builder and its palette system.
        let mut builder = TailwindBuilder::default();
        
        // 2. Register your custom "primary" color.
        builder.palettes.register_keyword(
            "primary".to_string(),
            "#9A66FF" // A purple color
        ).unwrap();


        // --- Test 1: Basic keyword ---
        
        // ACT: Parse a class name that uses the keyword.
        let pattern = ["primary"];
        let arbitrary = TailwindArbitrary::from(""); // A dummy arbitrary value
        let color = TailwindColor::parse(&pattern, &arbitrary).unwrap();

        // ASSERT: Check that the final CSS is correct.
        let css_output = color.get_properties(&builder);
        // 9A hex = 154, 66 hex = 102, FF hex = 255
        assert_eq!(css_output, "rgba(154, 102, 255, 1)");


        // --- Test 2: Keyword with an opacity modifier ---

        // ACT: Parse a class name with an opacity modifier.
        let pattern_with_alpha = ["primary/50"];
        let color_with_alpha = TailwindColor::parse(&pattern_with_alpha, &arbitrary).unwrap();

        // ASSERT: Check that the final CSS includes the correct alpha.
        let css_output_with_alpha = color_with_alpha.get_properties(&builder);
        assert_eq!(css_output_with_alpha, "rgba(154, 102, 255, 0.5)");
        
        println!("✅ Custom keyword tests passed!");
    }
}