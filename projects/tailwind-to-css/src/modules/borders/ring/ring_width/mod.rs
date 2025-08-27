use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    kind: UnitValue,
}

impl<T> From<T> for TailwindRingWidth
where
    T: Into<UnitValue>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

/// Generates the correct class name, e.g., `ring-4` or `ring-[3px]`.
impl Display for TailwindRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Special case for the default `ring` class, which has no suffix.
        if let UnitValue::Keyword(s) = &self.kind {
            if s == "<DEFAULT>" {
                return write!(f, "ring");
            }
        }
        write!(f, "ring-{}", self.kind)
    }
}

/// Generates the correct layered box-shadow CSS
impl TailwindInstance for TailwindRingWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Resolve the UnitValue to a CSS value, defaulting to px.
        let width = self.kind.get_properties(|f| format!("{}px", f));

        css_attributes! {
            "--tw-ring-offset-shadow" => "0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
            "--tw-ring-shadow" => format!("0 0 0 calc({} + var(--tw-ring-offset-width)) var(--tw-ring-color)", width),
            "box-shadow" => "var(--tw-ring-inset) var(--tw-ring-offset-shadow), var(--tw-ring-inset) var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)"
        }
    }
}

impl TailwindRingWidth {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = UnitValue::positive_parser("ring", Self::check_valid, true, false, false)(pattern, arbitrary)?;
        Ok(Self { kind })
    }

    /// Ring width doesn't have special keywords like medium/thick/thin, so we only
    /// include the standard CSS ones for validation.
    /// - https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#syntax

    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}