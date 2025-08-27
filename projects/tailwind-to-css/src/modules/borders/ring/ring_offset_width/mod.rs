use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    kind: UnitValue,
}

impl<T> From<T> for TailwindRingOffsetWidth
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

/// Generates the class name, e.g., `ring-offset-4`.
impl Display for TailwindRingOffsetWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.kind)
    }
}

/// Generates the exact CSS variables as tailwind v4.1.12
impl TailwindInstance for TailwindRingOffsetWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let width = self.kind.get_properties(|f| format!("{}px", f));

        css_attributes! {
            "--tw-ring-offset-width" => width,
            "--tw-ring-offset-shadow" => "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)"
        }
    }
}

impl TailwindRingOffsetWidth {
    /// <https://tailwindcss.com/docs/ring-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = UnitValue::positive_parser("ring-offset", Self::check_valid, true, false, false)(pattern, arbitrary)?;
        Ok(Self { kind })
    }

    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}