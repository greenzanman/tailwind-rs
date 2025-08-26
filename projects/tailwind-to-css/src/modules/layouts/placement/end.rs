use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindEnd {
    kind: UnitValue,
}

impl Display for TailwindEnd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "end-")
    }
}

impl TailwindInstance for TailwindEnd {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "inset-inline-end" => self.kind.get_properties_rem()
        }
    }
}

impl TailwindEnd {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = get_kind_px_full_auto_fract("end", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-end#syntax
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
