use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindResize {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindResize => "resize");

impl Display for TailwindResize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "resize-", |s| match s {
            "both" => KeywordClassFormat::CustomClassname("resize"),
            "horizontal" => KeywordClassFormat::AddAsSuffixCustom("x"),
            "vertical" => KeywordClassFormat::AddAsSuffixCustom("y"),

            keyword if TailwindResize::check_valid(keyword) => KeywordClassFormat::AddAsSuffix,
            
            _ => KeywordClassFormat::InvalidKeyword,
        })
    }
}

impl TailwindResize {
    /// https://tailwindcss.com/docs/resize
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_none() => StandardValue::from("both"),
            ["x"] => StandardValue::from("horizontal"),
            ["y"] => StandardValue::from("vertical"),
            _ => StandardValue::parser("resize", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/resize#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "block",
            "both",
            "horizontal",
            "inherit",
            "initial",
            "inline",
            "none",
            "revert",
            "unset",
            "vertical",
        ]);
        set.contains(mode)
    }
}
