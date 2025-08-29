use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScale {
    kind: NumericValue,
    axis: AxisXY,
}
impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.axis {
            AxisXY::N => self.kind.write_class_name(f, "scale-"),
            AxisXY::X => self.kind.write_class_name(f, "scale-x-"),
            AxisXY::Y => self.kind.write_class_name(f, "scale-y-"),
        }
    }
}

impl TailwindInstance for TailwindScale {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let scale = self.kind.get_properties(|f| (f/100.0).to_string());
        let transform = match self.axis {
            AxisXY::N => format!("scale({})", scale),
            AxisXY::X => format!("scaleX({})", scale),
            AxisXY::Y => format!("scaleY({})", scale),
        };
        css_attributes! {
            "transform" => transform,
        }
    }
}

// noinspection DuplicatedCode
impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = NumericValue::negative_parser("scale", |_| false)(rest, arbitrary, negative)?;
        Ok(TailwindScale { kind, axis })
    }
}
