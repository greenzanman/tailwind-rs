use super::*;

mod traits;

/// Used to represent those attributes that only have integers
#[derive(Debug, Clone)]
pub enum NumericValue {
    Number { n: f32, negative: bool, can_be_negative: bool },
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

impl NumericValue {
    pub fn get_properties(&self, number: impl FnOnce(&f32) -> String) -> String {
        match self {
            Self::Number { n, .. } => number(n),
            Self::Keyword(s) => s.to_string(),
            Self::Arbitrary(s) => s.get_properties(),
        }
    }
    pub fn write_negative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Number { n, can_be_negative, .. } if can_be_negative && n < 0.0 => write!(f, "-"),
            _ => write!(f, ""),
        }
    }
    /// Helper to write the Display classname for a NumericValue:
    /// - For NumericValue::Number, it will be {negative}{prefix}{abs(value)}. Example: "-order-4"
    /// - For NumericValue::Keyword, it will be {prefix}{value}. Example: "order-none"
    /// - For NumericValue::Arbitrary, it will be {prefix}{value}. Example: "mt-[4rem]"
    pub fn write_class_name(&self, f: &mut Formatter, prefix: &str) -> std::fmt::Result {
        match self {
            Self::Number { n, .. } => {
                self.write_negative(f)?;
                write!(f, "{}{}", prefix, n.abs())
            }
            Self::Keyword(s) => write!(f, "{}{}", prefix, s),
            Self::Arbitrary(s) => write!(f, "{}[{}]", prefix, s.get_properties()),
        }
    }
}

impl NumericValue {
    /// For parsing numbers that *can* be negative.
    pub fn negative_parser(
        id: &'static str,
        checker: impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary, Negative) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative| {
            let joined = pattern.join("-");
            match pattern {
                _ if checker(&joined) => Ok(Self::Keyword(joined)),
                [] => Self::parse_arbitrary(arbitrary),
                [n] => Self::parse_number(n, negative),
                _ => Err(TailwindError::syntax_error(format!("Unknown {} pattern", id))),
            }
        }
    }
    /// For parsing numbers that *cannot* be negative.
    pub fn positive_parser(
        id: &'static str,
        checker: impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| {
            let joined = pattern.join("-");
            match pattern {
                _ if checker(&joined) => Ok(Self::Keyword(joined)),
                [] => Self::parse_arbitrary(arbitrary),
                [n] => {
                    let i = TailwindArbitrary::from(*n).as_integer()?;
                    Ok(Self::Number { n: i as f32, negative: false, can_be_negative: false })
                },
                _ => Err(TailwindError::syntax_error(format!("Unknown {} pattern", id))),
            }
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    /// Helper for negative_parser
    pub fn parse_number(n: &str, negative: Negative) -> Result<Self> {
        let mut n = TailwindArbitrary::from(n).as_float()?;
        if negative.0 {
            n = -n
        }
        Ok(Self::Number { n, negative: negative.0, can_be_negative: true })
    }
}
