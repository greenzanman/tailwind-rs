use super::*;
use crate::StandardValue;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEase {
    kind: StandardValue,
}

impl Display for TailwindEase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ease-{}", self.kind)
    }
}

// Added implementation to generate the correct CSS attributes for v4.
impl TailwindInstance for TailwindEase {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let value = match self.kind.to_string().as_str() {
            "in" => "var(--ease-in)",
            "out" => "var(--ease-out)",
            "in-out" => "var(--ease-in-out)",
            "linear" => "linear",
            // Handles arbitrary values like `cubic-bezier(...)`
            _ => self.kind.get_properties(),
        };

        css_attributes! {
            "--tw-ease" => value,
            "transition-timing-function" => value
        }
    }
}


impl TailwindEase {
    /// https://tailwindcss.com/docs/transition-timing-function
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("ease", &Self::check_valid)(pattern, arbitrary)? })

    }
    
    /// Can be called if parse doesn't detect a keyword, to parse an arbitrary value.
    /// https://tailwindcss.com/docs/transition-timing-function#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    
    /// Returns Ok(keyword) if valid, Err otherwise.
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "",
            "in",
            "in-out",
            "out",
            "inherit",
            "initial",
            "linear",
            "revert",
            "step-end",
            "step-start",
            "unset",
        ]);
        set.contains(mode)
    }
}
