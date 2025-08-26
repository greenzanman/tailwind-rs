use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindJustifyContent {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindJustifyContent => "justify-content", {
    // Mapping keyword to the property value
    "between" => "space-between",
    "around" => "space-around",
    "evenly" => "space-evenly",
    "start" => "flex-start",
    "left" => "flex-start",
    "end" => "flex-end",
    "right" => "flex-end",
    "center-safe" => "safe center",
    "end-safe" => "safe end",
    // For any other valid keyword, return it as is.
});

impl Display for TailwindJustifyContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                // Match the mapped property keywords to their different tailwind names
                "flex-start" => write!(f, "justify-start"),
                "flex-end" => write!(f, "justify-end"),
                "center" => write!(f, "justify-center"),
                "space-between" => write!(f, "justify-between"),
                "space-around" => write!(f, "justify-around"),
                "space-evenly" => write!(f, "justify-evenly"),
                "safe center" => write!(f, "justify-center-safe"),
                "safe end" => write!(f, "justify-end-safe"),
                // Other keywords like 'baseline' would simply be 'justify-baseline'
                _ => write!(f, "justify-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "justify-content-"),
        }
    }
}

impl TailwindJustifyContent {
    /// <https://tailwindcss.com/docs/justify-content>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        // 1. Parse the input as before.
        let mut kind = StandardValue::parser("justify-content", &Self::check_valid)(pattern, arbitrary)?;

        // 2. Then, if the result is a keyword, map it to its canonical form.
        if let StandardValue::Keyword(s) = &kind {
            kind = StandardValue::Keyword(Self::map_keyword(s).to_string());
        }

        Ok(Self { kind })
    }
    /// dispatch to [justify-content](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        StandardValue::parse_arbitrary(arbitrary).map(|kind| Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "start",
            "end",
            "center",
            "between",
            "around",
            "evenly",
            "stretch",
            "baseline",
            "normal",
            "flex-start",       // alias for start
            "flex-end",         // alias for end
            "space-around",    // alias for around
            "space-between",   // alias for between
            "space-evenly",    // alias for evenly
            "start",
            "stretch",
            "end-safe",
            "center-safe",
            // Extended syntax
            "inherit",
            "initial",
            "left",
            "revert",
            "right",
            "unset",
        ]);
        set.contains(mode)
    }
}
