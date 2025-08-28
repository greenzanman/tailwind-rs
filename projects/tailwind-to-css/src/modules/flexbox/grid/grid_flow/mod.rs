use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindGridFlow {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindGridFlow => "grid-auto-flow");

impl Display for TailwindGridFlow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "grid-flow-", |s| match s {
            "column dense" => KeywordClassFormat::AddAsSuffixCustom("col-dense"),
            "row dense" => KeywordClassFormat::AddAsSuffixCustom("row-dense"),

            keyword if TailwindGridFlow::check_valid(keyword) => KeywordClassFormat::AddAsSuffix,

            _ => KeywordClassFormat::InvalidKeyword,
        })
    }
}

impl TailwindGridFlow {
    /// <https://tailwindcss.com/docs/grid-auto-flow>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["row"] => StandardValue::from("row"),
            ["col"] => StandardValue::from("column"),
            ["dense"] => StandardValue::from("dense"),
            ["col", "dense"] => StandardValue::from("column dense"),
            ["row", "dense"] => StandardValue::from("row dense"),
            _ => StandardValue::parser("grid-auto", &Self::check_valid)(pattern, arbitrary)?,
        };
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["column", "row", "dense", "row dense", "column dense", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
