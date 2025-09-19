use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideStyle {
    kind: String,
}

impl From<&str> for TailwindDivideStyle {
    /// Creates a new TailwindDivideStyle from a style keyword like "solid".
    fn from(style: &str) -> Self {
        Self { kind: style.to_string() }
    }
}

impl Display for TailwindDivideStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "divide-{}", self.kind)
    }
}

impl TailwindInstance for TailwindDivideStyle {
    fn inlineable(&self) -> bool {
        // TailwindDivideStyle cannot be inlined bc it generates a class with a nested rule.
        false
    }

    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Create the inner attributes for the nested rule.
        let inner_attrs = css_attributes! {
            // this CSS variable can be used by other utilities.
            "--tw-border-style" => self.kind.clone(),
            "border-style" => self.kind.clone()
        };

        // Create the top-level object to hold the nested rule.
        let mut top_level_attrs = CssAttributes::default();

        // Insert the nested rule with the correct child selector.
        let selector = ":where(& > :not(:last-child))".to_string();
        top_level_attrs.insert_nested(selector, inner_attrs);

        top_level_attrs
    }
}
