use super::*;

impl Display for SizingUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fit => write!(f, "fit"),
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Screen => write!(f, "screen"),
            Self::Fraction(numerator, denominator) => write!(f, "{}/{}", numerator, denominator),
            Self::Length(x) => write!(f, "[{}]", x),
        }
    }
}

impl SizingUnit {
    fn get_attribute(&self, kind: TailwindSizingKind) -> String {
        let is_width = kind.is_width();
        let is_width_str = if is_width { "vw" } else { "vh" };

        match (self, kind) {
            // `None` logic is now combined in the match
            (Self::None, TailwindSizingKind::MaxWidth | TailwindSizingKind::MaxHeight) => {
                "none".to_string()
            }
            (Self::None, _) => "0px".to_string(),

            // Other cases
            (Self::Min, _) => "min-content".to_string(),
            (Self::Max, _) => "max-content".to_string(),
            (Self::Fit, _) => "fit-content".to_string(),
            (Self::Auto, _) => "auto".to_string(),
            (Self::Full, _) => "100%".to_string(),
            (Self::Screen, _) => format!("100{}", is_width_str),
            (Self::Fraction(numerator, denominator), _) => {
                format!("{}%", (*numerator as f32 / *denominator as f32) * 100.0)
            }
            (Self::Length(x), _) => format!("{}", x),
        }
    }
}

impl TailwindSizingKind {
    fn is_width(self) -> bool {
        matches!(self, Self::Width | Self::MinWidth | Self::MaxWidth)
    }
}

impl Display for TailwindSizingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Width => f.write_str("width"),
            Self::MinWidth => f.write_str("min-width"),
            Self::MaxWidth => f.write_str("max-width"),
            Self::Height => f.write_str("height"),
            Self::MinHeight => f.write_str("min-height"),
            Self::MaxHeight => f.write_str("max-height"),
        }
    }
}

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.kind, &self.size) {
            // Handles `max-w-none` and `max-h-none`
            (TailwindSizingKind::MaxWidth | TailwindSizingKind::MaxHeight, SizingUnit::None) => {
                write!(f, "{}-none", self.kind)
            }
            // Handles `w-none`, `h-none`, etc. by turning them into `w-0`, `h-0`
            (_, SizingUnit::None) => write!(f, "{}-0", self.kind),

            // Handles a parsed `w-0`, `h-0`, etc.
            (_, SizingUnit::Length(val)) if val.is_zero() => {
                write!(f, "{}-0", self.kind)
            }

            // Handles all other cases like `w-full` or `w-1/2`
            _ => write!(f, "{}-{}", self.kind, self.size),
        }
    }
}

impl TailwindInstance for TailwindSizing {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.kind.to_string();
        let width = self.size.get_attribute(self.kind);
        css_attributes! {
            class => width
        }
    }
}
