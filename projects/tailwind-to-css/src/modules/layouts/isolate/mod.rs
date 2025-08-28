use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindIsolation {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindIsolation => "isolation");

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_class(f, "isolation-", |s| match s {
            // Special Case: The output is just the keyword itself, without the "isolation-" prefix.
            "isolate" => KeywordClassFormat::CustomClassname("isolate"),

            // General Rule: The output requires a prefix.
            keyword if TailwindIsolation::check_valid(keyword) => KeywordClassFormat::AddAsSuffix,
            
            // Anything else is invalid.
            _ => KeywordClassFormat::InvalidKeyword,
        })
    }
}

impl TailwindIsolation {
    /// https://tailwindcss.com/docs/isolation
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("isolate", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/isolation#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "auto", "isolate", // Global values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
