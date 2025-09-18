use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundImage {
    // The type of image/gradient
    kind: BgImageKind,
    // The value (e.g., direction, angle, url). Optional for defaults like `bg-radial`.
    value: Option<UnitValue>,
    // Flag for negative angles
    is_negative: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum BgImageKind {
    None,
    Url,
    Linear,
    Radial,
    Conic,
    RepeatingLinear,
    RepeatingRadial,
    RepeatingConic,
}

impl Display for TailwindBackgroundImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_negative {
            write!(f, "-")?;
        }

        match (&self.kind, &self.value) {
            (BgImageKind::None, _) => write!(f, "bg-none"),
            (BgImageKind::Url, Some(val)) => write!(f, "bg-[url({})]", val),
            (BgImageKind::Linear, Some(val)) => write!(f, "bg-linear-{}", val),
            (BgImageKind::Radial, Some(val)) => write!(f, "bg-radial-{}", val),
            (BgImageKind::Conic, Some(val)) => write!(f, "bg-conic-{}", val),
            (BgImageKind::Linear, None) => write!(f, "bg-linear"),
            (BgImageKind::Radial, None) => write!(f, "bg-radial"),
            (BgImageKind::Conic, None) => write!(f, "bg-conic"),
            (BgImageKind::RepeatingLinear, Some(val)) => write!(f, "bg-[repeating-linear-gradient({})]", val),
            (BgImageKind::RepeatingRadial, Some(val)) => write!(f, "bg-[repeating-radial-gradient({})]", val),
            (BgImageKind::RepeatingConic, Some(val)) => write!(f, "bg-[repeating-conic-gradient({})]", val),
            _ => Ok(()), // Should not happen
        }
    }
}

impl TailwindInstance for TailwindBackgroundImage {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            BgImageKind::None => css_attributes! { "background-image" => "none" },
            BgImageKind::Url => {
                let url_value = self.value.as_ref().unwrap().to_string();
                css_attributes! { "background-image" => format!("url({})", url_value) }
            }
            BgImageKind::RepeatingLinear | BgImageKind::RepeatingRadial | BgImageKind::RepeatingConic => {
                let arbitrary_gradient_val = self.value.as_ref().unwrap().to_string().replace('_', " ");
                
                let gradient_type = match self.kind {
                    BgImageKind::RepeatingLinear => "repeating-linear-gradient",
                    BgImageKind::RepeatingRadial => "repeating-radial-gradient",
                    BgImageKind::RepeatingConic => "repeating-conic-gradient",
                    _ => unreachable!(),
                };
                css_attributes! { "background-image" => format!("{}({})", gradient_type, arbitrary_gradient_val) }
            }
            BgImageKind::Linear | BgImageKind::Radial | BgImageKind::Conic => {
                let gradient_type = match self.kind {
                    BgImageKind::Linear => "linear-gradient",
                    BgImageKind::Radial => "radial-gradient",
                    BgImageKind::Conic => "conic-gradient",
                    _ => unreachable!(),
                };

                let direction_or_angle = match self.value.as_ref() {
                    // No value, e.g., `bg-radial` -> `in oklab`
                    None => "in oklab".to_string(),
                    // Directional keyword, e.g., `to-t` -> `to top in oklab`
                    Some(UnitValue::Keyword(k)) if k.starts_with("to-") => {
                        let direction = k.strip_prefix("to-").unwrap_or(k);
                        let mut full_direction = String::new();
                        if direction.contains('t') { full_direction.push_str("top "); }
                        if direction.contains('b') { full_direction.push_str("bottom "); }
                        if direction.contains('l') { full_direction.push_str("left "); }
                        if direction.contains('r') { full_direction.push_str("right "); }
                        format!("to {} in oklab", full_direction.trim())
                    }
                    // An angle or other arbitrary value, e.g., `45deg`
                    Some(value) => {
                        // Get the inner value of an arbitrary, e.g., `45deg` from `[45deg]`
                        let inner_value = match value {
                             UnitValue::Arbitrary(a) => a.get_properties(),
                             _ => value.to_string()
                        };
                    
                        if self.is_negative {
                            format!("calc({} * -1)", inner_value)
                        } else {
                            format!("{} in oklab", inner_value)
                        }
                    }
                };

                // For arbitrary angles, the position is added as a fallback variable
                let image = match self.value.as_ref() {
                     Some(UnitValue::Arbitrary(_)) | Some(UnitValue::Number {..}) | Some(UnitValue::Length(_)) => {
                        format!("{}(var(--tw-gradient-stops, {}))", gradient_type, direction_or_angle)
                     },
                     _ => format!("{}(var(--tw-gradient-stops))", gradient_type)
                };

                css_attributes! {
                    "--tw-gradient-position" => direction_or_angle,
                    "background-image" => image
                }
            }
        }
    }
}

impl TailwindBackgroundImage {
    /// Should only be called if arbitrary.as_str().starts_with("url(") || arbitrary.as_str().contains("gradient(")
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: Negative) -> Result<Self> {
        let (kind, rest) = match pattern {
            ["none"] => (BgImageKind::None, &pattern[1..]),
            ["gradient", rest @ ..] | ["linear", rest @ ..] => (BgImageKind::Linear, rest),
            ["radial", rest @ ..] => (BgImageKind::Radial, rest),
            ["conic", rest @ ..] => (BgImageKind::Conic, rest),
            [] if arbitrary.as_str().starts_with("url(") => (BgImageKind::Url, pattern),
            [] if arbitrary.as_str().starts_with("repeating-linear-gradient(") => (BgImageKind::RepeatingLinear, pattern),
            [] if arbitrary.as_str().starts_with("repeating-radial-gradient(") => (BgImageKind::RepeatingRadial, pattern),
            [] if arbitrary.as_str().starts_with("repeating-conic-gradient(") => (BgImageKind::RepeatingConic, pattern),
            _ => return syntax_error!("Unknown background-image pattern: {}", pattern.join("-")),
        };

        let value = match kind {
            BgImageKind::None => None,
            BgImageKind::Url => {
                let url = arbitrary.as_str().strip_prefix("url(").and_then(|s| s.strip_suffix(')')).unwrap_or("");
                Some(UnitValue::Keyword(url.to_string()))
            }
            BgImageKind::RepeatingLinear | BgImageKind::RepeatingRadial | BgImageKind::RepeatingConic => {
                let prefix = match kind {
                    BgImageKind::RepeatingLinear => "repeating-linear-gradient(",
                    BgImageKind::RepeatingRadial => "repeating-radial-gradient(",
                    BgImageKind::RepeatingConic => "repeating-conic-gradient(",
                    _ => unreachable!(),
                };
                let content = arbitrary.as_str().strip_prefix(prefix).and_then(|s| s.strip_suffix(')')).unwrap_or("");
                Some(UnitValue::Keyword(content.to_string()))
            }
            BgImageKind::Linear 
            | BgImageKind::Radial 
            | BgImageKind::Conic => {
                if rest.is_empty() && arbitrary.is_none() {
                    // This is a default gradient, like `bg-radial`
                    None
                } else {
                    let joined = rest.join("-");
                    if joined.starts_with("to-") {
                        // It's a directional keyword like `to-t` or `to-b-r`
                        Some(UnitValue::Keyword(joined.replace("-b-r", "-br").replace("-b-l", "-bl").replace("-t-r", "-tr").replace("-t-l", "-tl")))
                    } else {
                        // Otherwise, parse as a number, angle, or other length value.
                        // The `is_length: false` flag tells the parser to prefer angles over lengths.
                        Some(UnitValue::negative_parser("bg-image", |_| false, false, false, false)(rest, arbitrary, neg)?)
                    }
                }
            }
        };

        Ok(Self { kind, value, is_negative: neg.0 })
    }
}
