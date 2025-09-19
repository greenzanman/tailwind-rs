use crate::NumericValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideWidth {
    axis: AxisXY, // true for X, false for Y
    kind: NumericValue,
}

impl Display for TailwindDivideWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Handle the default case (e.g., "divide-x") vs. explicit width ("divide-x-2")
        self.axis.write_xy(f, "divide", &self.kind)
    }
}

impl TailwindInstance for TailwindDivideWidth {
    fn inlineable(&self) -> bool {
        // This utility generates a nested rule (:where(& > :not(:last-child))) and cannot be inlined.
        false
    }

    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let width = self.kind.get_properties(|f| format!("{}px", f));
        
        let inner_attrs = match self.axis {
            AxisXY::X => {
                let var_name = "--tw-divide-x-reverse";
                css_attributes! {
                    var_name => "0",
                    "border-inline-style" => "var(--tw-border-style)",
                    "border-inline-start-width" => format!("calc({} * var({}))", width, var_name),
                    "border-inline-end-width" => format!("calc({} * calc(1 - var({})))", width, var_name)
                }
            },
            AxisXY::Y => {
                let var_name = "--tw-divide-y-reverse";
                css_attributes! {
                    var_name => "0",
                    "border-top-style" => "var(--tw-border-style)",
                    "border-bottom-style" => "var(--tw-border-style)",
                    "border-top-width" => format!("calc({} * var({}))", width, var_name),
                    "border-bottom-width" => format!("calc({} * calc(1 - var({})))", width, var_name)
                }
            },
            AxisXY::N => unreachable!(),
        };

        // Create the top-level object to hold the nested rule
        let mut top_level_attrs = CssAttributes::default();
        let selector = ":where(& > :not(:last-child))".to_string();
        top_level_attrs.insert_nested(selector, inner_attrs);

        top_level_attrs
    }
}


impl TailwindDivideWidth {
    /// Parses divide-width utilities like `divide-x`, `divide-y-2`, `divide-x-[3rem]`.
    /// https://tailwindcss.com/docs/divide-width
    pub fn parse(width_pattern: &[&str], arbitrary: &TailwindArbitrary, axis_is_x: bool) -> Result<Self> {
        // If the width pattern and arbitrary value are empty, it's a default width.
        if width_pattern.is_empty() && arbitrary.is_none() {
            Ok(Self {
                axis: AxisXY::from(axis_is_x),
                // Default width is 1px.
                kind: NumericValue::Number { n: 1.0, negative: false, can_be_negative: false },
            })
        } else {
            // Otherwise, parse the explicit width.
            //  - checker for keywords is false bc there no keywords for divide-width
            let kind = NumericValue::positive_parser("divide-width", |_| false)(width_pattern, arbitrary)?;
            Ok(Self { axis: AxisXY::from(axis_is_x), kind })
        }
    
    }
}
