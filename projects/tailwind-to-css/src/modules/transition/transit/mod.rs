use super::*;

#[derive(Clone, Debug)]
enum Transition {
    None,
    All,
    Default,
    Colors,
    Opacity,
    Shadow,
    Transform,
    Arbitrary(TailwindArbitrary),
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTransition {
    kind: Transition,
}

impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-none"),
            Self::All => write!(f, "-all"),
            Self::Default => write!(f, ""),
            Self::Colors => write!(f, "-colors"),
            Self::Opacity => write!(f, "-opacity"),
            Self::Shadow => write!(f, "-shadow"),
            Self::Transform => write!(f, "-transform"),
            Self::Arbitrary(g) => write!(f, "-[{}]", g.get_class()),
        }
    }
}

impl Display for TailwindTransition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "transition{}", self.kind)
    }
}

impl TailwindInstance for TailwindTransition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // https://tailwindcss.com/docs/transition-property
        match &self.kind {
            Transition::None => css_attributes! {
                "transition-property" => "none"
            },
            Transition::All => css_attributes! {
                "transition-property" => "all",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Default => css_attributes! {
                "transition-property" => "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to, opacity, box-shadow, transform, translate, scale, rotate, filter, -webkit-backdrop-filter, backdrop-filter, display, visibility, content-visibility, overlay, pointer-events",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Colors => css_attributes! {
                "transition-property" => "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Opacity => css_attributes! {
                "transition-property" => "opacity",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Shadow => css_attributes! {
                "transition-property" => "box-shadow",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Transform => css_attributes! {
                "transition-property" => "transform, translate, scale, rotate",
                "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
            },
            Transition::Arbitrary(a) => {
                let value = a.get_properties();
                css_attributes! {
                    "transition-property" => value,
                    "transition-timing-function" => "var(--tw-ease, var(--default-transition-timing-function))",
                    "transition-duration" => "var(--tw-duration, var(--default-transition-duration))"
                }
            }
        }
    }
}

impl TailwindTransition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse_arbitrary(arbitrary)? })
    }
}

impl Transition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let t = match pattern {
            [] if arbitrary.is_none() => Self::Default,
            [] => Self::parse_arbitrary(arbitrary)?,
            ["none"] => Self::None,
            ["all"] => Self::All,
            ["colors"] => Self::Colors,
            ["opacity"] => Self::Opacity,
            ["shadow"] => Self::Shadow,
            ["transform"] => Self::Transform,
            _ => return syntax_error!("Unknown transition instructions: {}", pattern.join("-")),
        };
        Ok(t)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}
