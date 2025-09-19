use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindDivideReverse {
    axis: bool, // true for X, false for Y
}

impl From<bool> for TailwindDivideReverse {
    fn from(axis: bool) -> Self {
        Self { axis }
    }
}

impl Display for TailwindDivideReverse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            true => write!(f, "divide-x-reverse"),
            false => write!(f, "divide-y-reverse"),
        }
    }
}

impl TailwindInstance for TailwindDivideReverse {
    fn inlineable(&self) -> bool {
        false // This generates a class, it cannot be inlined.
    }

    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            true => "--tw-divide-x-reverse",
            false => "--tw-divide-y-reverse",
        };

        // Create the inner attributes that set the variable to 1
        let inner_attrs = css_attributes! {
            class => "1"
        };
        
        // Create the top-level object
        let mut top_level_attrs = CssAttributes::default();

        // Insert the nested rule
        let selector = ":where(& > :not(:last-child))".to_string();
        top_level_attrs.insert_nested(selector, inner_attrs);

        top_level_attrs
    }
}