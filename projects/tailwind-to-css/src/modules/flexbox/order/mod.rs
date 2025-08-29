use super::*;
use crate::NumericValue;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailWindOrder {
    kind: NumericValue,
}

impl Display for TailWindOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            NumericValue::Number { n, .. } => match *n as i32 {
                -9999 => write!(f, "order-first"),
                9999 => write!(f, "order-last"),
                _ => {
                    self.kind.write_class_name(f, "order-")
                }
            },
            // For Keyword and Arbitrary variants
            _ => {
                self.kind.write_class_name(f, "order-")
            },
        }
    }
}

impl TailwindInstance for TailWindOrder {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "order" => self.kind
        }
    }
}

impl TailWindOrder {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = match pattern {
            ["none"] => NumericValue::from(0i32),
            ["first"] => NumericValue::from(-9999i32),
            ["last"] => NumericValue::from(9999i32),
            [s] if Self::check_valid(s) => NumericValue::Keyword(s.to_string()),
            _ => NumericValue::negative_parser("order", &Self::check_valid)(pattern, arbitrary, negative)?,
        };
        Ok(Self { kind })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: NumericValue::parse_arbitrary(arbitrary)? })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
