pub(crate) mod divide_color;
pub(crate) mod divide_reverse;
pub(crate) mod divide_style;
pub(crate) mod divide_width;

use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindDivide {}

impl TailwindDivide {
    /// Parse the instructions starting with `divide`.
    pub fn adapt(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {

        let out = match pattern {
            // --- Divide Width ---
             // https://tailwindcss.com/docs/divide-width
            ["x", "reverse"] => TailwindDivideReverse::from(true).boxed(), // true = X-axis
            ["y", "reverse"] => TailwindDivideReverse::from(false).boxed(), // false = Y-axis

            // Catches all other width-related classes:
            // e.g., divide-x, divide-y, divide-x-2, divide-y-[3px]
            ["x", rest @ ..] => TailwindDivideWidth::parse(rest, arbitrary, true)?.boxed(),
            ["y", rest @ ..] => TailwindDivideWidth::parse(rest, arbitrary, false)?.boxed(),

            // --- Divide Style (Alias to TailwindBorderStyle) ---
            // https://tailwindcss.com/docs/border-style#setting-the-divider-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "hidden" | "none")] => TailwindDivideStyle::from(*s).boxed(),


            // --- Divide Color  ---
            // https://tailwindcss.com/docs/border-color#divider-between-children
            // If not a width, reverse, or style, it's a color
            _ => TailwindDivideColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
}