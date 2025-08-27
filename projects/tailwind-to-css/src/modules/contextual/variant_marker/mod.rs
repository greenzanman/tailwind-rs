use crate::{CssAttributes, TailwindBuilder, TailwindInstance, TailwindArbitrary, Result, StandardValue};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[doc=include_str!("readme.md")]
#[derive(Debug)]
pub struct TailwindVariantMarker {
    kind: StandardValue,
}



impl Display for TailwindVariantMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Return name as traced name
        write!(f, "{}", self.kind)
    }
}

impl TailwindInstance for TailwindVariantMarker {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Give empty css properties
        CssAttributes::default()
        // !todo: this will cause the obfuscated classnames to be indentical
        // !todo: During obfuscation mode, the variants need to know which classnames are the variant markers...
    }
}

impl TailwindVariantMarker {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("variant-marker", &Self::check_valid)(pattern, arbitrary)? })
    }

    pub fn check_valid(mode: &str) -> bool {
        // Sibling and parent markers
        let set = BTreeSet::from_iter(vec!["peer", "group"]);
        set.contains(mode)
    }
}