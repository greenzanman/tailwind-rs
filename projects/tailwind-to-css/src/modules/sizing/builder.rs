use super::*;

impl SizingUnit {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let px = |x| Ok(Self::Length(LengthUnit::px(x)));
        match pattern {
            ["none"] => Ok(Self::None),
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Max),
            ["auto"] => Ok(Self::Auto),
            ["full"] => Ok(Self::Full),
            ["fit"] => Ok(Self::Fit),
            ["screen"] => Ok(Self::Screen),
            ["0"] => px(0.0),
            ["px"] => px(1.0),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown sizing instructions: {}", pattern.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_fraction(arbitrary).or_else(|_| Self::maybe_no_unit(arbitrary)).or_else(|_| Self::maybe_length(arbitrary)).or_else(|_| Self::maybe_arbitrary(arbitrary))
        // ! Does not match for the following test cases:
        // parse_arbitrary: input=TailwindArbitrary { inner: "xs" }
        //  - aka for max-w-sm and min-w-sm
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length_or_fraction()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |x| Ok(Self::Length(LengthUnit::rem(x)));
        rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_fraction(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self::Fraction(a, b))
    }
    #[inline]
    fn maybe_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        println!("maybe_arbitrary: input={:?}", arbitrary.as_str());
        Ok(Self::Arbitrary(arbitrary.clone()))
    }
}

impl TailwindSizing {
    #[inline]
    pub fn parse_width(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Width, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_max(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxWidth, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_min(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinWidth, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Height, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_max(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxHeight, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_min(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinHeight, size: SizingUnit::parse(pattern, arbitrary)? })
    }
}
