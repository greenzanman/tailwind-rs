mod builder;
mod display;

use super::*;

#[derive(Copy, Clone, Debug)]
enum TailwindSizingKind {
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
}

#[derive(Clone, Debug)]
enum SizingUnit {
    None,
    Min,
    Max,
    Fit,
    Auto,
    Full,
    Screen,
    Fraction(usize, usize),
    Length(LengthUnit),
    Arbitrary(TailwindArbitrary),
}

#[doc = include_str!("sizing.md")]
#[derive(Clone, Debug)]
pub struct TailwindSizing {
    kind: TailwindSizingKind,
    size: SizingUnit,
}
