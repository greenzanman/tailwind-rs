use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindLeading {
    kind: UnitValue,
}

// A convenience implementation to easily create instances.
impl<T> From<T> for TailwindLeading
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}


impl Display for TailwindLeading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "leading-{}", self.kind)
    }
}

impl TailwindInstance for TailwindLeading {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Map the keywords to their specific values.
        let line_height = match &self.kind {
            UnitValue::Keyword(k) => match k.as_str() {
                // Tailwind v3 keywords that map to unitless values
                "none" => "1.0",
                "tight" => "1.25",
                "snug" => "1.375",
                "normal" => "1.5",
                "relaxed" => "1.625",
                "loose" => "2.0",
                // Standard CSS keyword
                "default" => "normal",
                _ => "1.5", // Default
            }.to_string(),
            // For UnitValue, use get_properties_rem to convert numbers to rem.
            _ => self.kind.get_properties_rem(),
        };

        css_attributes! {
            "line-height" => line_height
        }
    }
}

impl TailwindLeading {
    /// https://v3.tailwindcss.com/docs/line-height
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            kind: UnitValue::positive_parser(
                "leading",
                Self::check_valid,
                true,  
                true,  
                false, 
            )(pattern, arbitrary)?,
        })
    }

    /// Checks for valid line-height keywords
    pub fn check_valid(mode: &str) -> bool {
        // "default" is from https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#normal
        let set = BTreeSet::from_iter([
            "none", "tight", "snug", "normal", "relaxed", "loose", "default",
        ]);
        set.contains(mode)
    }
}