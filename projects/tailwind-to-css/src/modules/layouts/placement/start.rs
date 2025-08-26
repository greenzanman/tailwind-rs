use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindStart {
    kind: UnitValue,
}

impl Display for TailwindStart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "start-")
    }
}

impl TailwindInstance for TailwindStart {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "start" => self.kind.get_properties_rem()
        }
    }
}

impl TailwindStart {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = get_kind_px_full_auto_fract("start", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-start#syntax
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}
