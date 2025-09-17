use super::*;
use crate::StandardValue;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEase {
    kind: StandardValue,
}
crate::macros::sealed::keyword_instance!(TailwindEase => "transition-timing-function",
    {
        "in" => "ease-in",
        "in-out" => "ease-in-out",
        "out" => "ease-out",
    }
);

impl Display for TailwindEase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ease-{}", self.kind)
    }
}

impl TailwindEase {
    /// https://tailwindcss.com/docs/transition-timing-function
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let value = &StandardValue::parser("ease", &|s| Self::check_valid(s).is_ok())(pattern, arbitrary)?.to_string();
        Ok(TailwindEase::from(value))
    }
    
    /// https://tailwindcss.com/docs/transition-timing-function#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    
    /// Returns Ok(keyword) if valid, Err otherwise.
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function#syntax
    pub fn check_valid(mode: &str) -> Result<&str> {
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
        if set.contains(mode) {
            Ok(mode)
        } else {
            syntax_error!("Invalid ease keyword: {}", mode)
        }
    }
}
