use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontSize {
    kind: UnitValue,
}


impl<T> From<T> for TailwindFontSize
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self {
            kind: kind.into(),
        }
    }
}

impl Display for TailwindFontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            // ! Official Tailwind would not allow Number (ex: text-5 not valid) 
            UnitValue::Number{ n, .. } => write!(f, "text-{}", n),  // ex: text-5
            UnitValue::Length(s) => write!(f, "text-{}", s),  // ex: text-[2.3rem]
            UnitValue::Keyword(s) => {write!(f, "text-{}", s)}  // ex: text-sm
            UnitValue::Arbitrary(s) => write!(f, "text-{}", s),
        }
    }
}

impl TailwindInstance for TailwindFontSize {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            UnitValue::Keyword(s) => {
                // Get font-size and line-height properties for built-in Keywords
                return ctx.fonts.get_size(s).get_properties();
                
            }
            _ => {
                css_attributes! {
                    "font-size" => self.kind.get_properties_rem(),
                }
            }

        }
    }
}

impl TailwindFontSize {
    // https://tailwindcss.com/docs/font-size
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = UnitValue::positive_parser("text-size", Self::check_valid_keyword, true, false, false)(pattern, arbitrary)?;
        Ok(Self { kind })
    }

    pub fn check_valid_keyword(mode: &str) -> bool {
        ["xs", "sm", "base", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl"].contains(&mode)
    }
}
