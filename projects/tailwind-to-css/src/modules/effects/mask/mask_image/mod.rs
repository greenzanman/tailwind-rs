use super::*;

// This new file would live alongside your other rule files.
// E.g., `src/rules/mask_image.rs`

#[derive(Clone, Debug)]
pub struct TailwindMaskImage {
    // The type of image/gradient for the mask
    kind: MaskImageKind,
    // The value (e.g., direction, angle).
    value: Option<UnitValue>,
    // Flag for negative angles
    is_negative: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum MaskImageKind {
    None,
    Linear,
    Radial,
    Conic,
    // Add Url, RepeatingLinear, etc. as needed in the future
}

impl Display for TailwindMaskImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_negative {
            write!(f, "-")?;
        }

        match (&self.kind, &self.value) {
            (MaskImageKind::None, _) => write!(f, "mask-none"),
            (MaskImageKind::Linear, Some(val)) => write!(f, "mask-linear-{}", val),
            (MaskImageKind::Radial, Some(val)) => write!(f, "mask-radial-{}", val),
            (MaskImageKind::Conic, Some(val)) => write!(f, "mask-conic-{}", val),
            (MaskImageKind::Linear, None) => write!(f, "mask-linear"),
            (MaskImageKind::Radial, None) => write!(f, "mask-radial"),
            (MaskImageKind::Conic, None) => write!(f, "mask-conic"),
            _ => Ok(()),
        }
    }
}

impl TailwindInstance for TailwindMaskImage {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let (prefix, property) = ("--tw-mask", "mask-image");

        match &self.kind {
            MaskImageKind::None => css_attributes! { property => "none" },
            MaskImageKind::Linear | MaskImageKind::Radial | MaskImageKind::Conic => {
                let gradient_type = match self.kind {
                    MaskImageKind::Linear => "linear-gradient",
                    MaskImageKind::Radial => "radial-gradient",
                    MaskImageKind::Conic => "conic-gradient",
                    _ => unreachable!(),
                };

                let direction_or_angle = match self.value.as_ref() {
                    None => "".to_string(), // Default direction/shape is often empty
                    Some(UnitValue::Keyword(k)) if k.starts_with("to-") => {
                        // ... logic to convert "to-t" to "to top" ...
                        // This can be copied from your TailwindBackgroundImage impl
                        k.replace("to-", "to ").replace("-", " ")
                    }
                    Some(value) => {
                        let inner_value = match value {
                            UnitValue::Arbitrary(a) => a.get_properties(),
                            _ => value.to_string(),
                        };
                        if self.is_negative {
                            format!("calc({} * -1)", inner_value)
                        } else {
                            inner_value
                        }
                    }
                };

                // The composable gradient that uses CSS variables
                let image = format!("{}({})", gradient_type, format!("{}, var({}-stops))", direction_or_angle, prefix));

                css_attributes! {
                    format!("{}-position", prefix) => direction_or_angle,
                    property => image
                }
            }
        }
    }
}

impl TailwindMaskImage {
    /// Parses `mask-none`, `mask-linear`, `mask-radial`, `mask-[...gradient...]`
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: Negative) -> Result<Self> {
        let (kind, rest) = match pattern {
            ["none"] => (MaskImageKind::None, &pattern[1..]),
            ["linear", rest @ ..] => (MaskImageKind::Linear, rest),
            ["radial", rest @ ..] => (MaskImageKind::Radial, rest),
            ["conic", rest @ ..] => (MaskImageKind::Conic, rest),
            // Handle arbitrary gradients
            [] if arbitrary.as_str().contains("gradient") => {
                // For arbitrary, we can treat it like a simple linear gradient for parsing purposes
                (MaskImageKind::Linear, pattern)
            }
            _ => return syntax_error!("Unknown mask-image pattern: {}", pattern.join("-")),
        };

        let value = match kind {
            MaskImageKind::None => None,
            _ => { // All gradient types
                if arbitrary.is_some() {
                    // For mask-[radial-gradient(...)], the full value is in `arbitrary`
                     let grad_content = arbitrary.as_str()
                        .replace('_', " ")
                        .trim_start_matches("linear-gradient(")
                        .trim_start_matches("radial-gradient(")
                        .trim_start_matches("conic-gradient(")
                        .trim_end_matches(')')
                        .to_string();
                    Some(UnitValue::Keyword(grad_content))
                } else if rest.is_empty() {
                    None // e.g., `mask-linear`
                } else {
                    Some(UnitValue::negative_parser("mask-image", |_| false, false, false, false)(rest, arbitrary, neg)?)
                }
            }
        };

        Ok(Self { kind, value, is_negative: neg.0 })
    }
}