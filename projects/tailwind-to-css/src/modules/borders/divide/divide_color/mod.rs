use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDivideColor {
    // reuse logic of TailwindBorderColor
    border_color_instance: TailwindBorderColor,
}

impl TailwindDivideColor {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            border_color_instance: TailwindBorderColor::parse(pattern, arbitrary)?,
        })
    }
}

impl Display for TailwindDivideColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Convert the inner border classname (e.g., "border-blue-500")
        // to a divide classname ("divide-blue-500").
        let border_class = self.border_color_instance.to_string();
        if let Some(color_part) = border_class.strip_prefix("border-") {
            write!(f, "divide-{}", color_part)
        } else {
            // Fallback for cases that might not start with "border-"
            write!(f, "divide-{}", border_class)
        }
    }
}

impl TailwindInstance for TailwindDivideColor {
    fn inlineable(&self) -> bool {
        false
    }

    fn attributes(&self, builder: &TailwindBuilder) -> CssAttributes {
        // 1. Get the flat attributes from the wrapped TailwindBorderColor instance.
        // This will contain a "border-color" property.
        let border_attrs = self.border_color_instance.attributes(builder);

        // 2. Create the inner attributes for our nested rule.
        // We assume the border_attrs contains the property we need.
        let inner_attrs = border_attrs;

        // 3. Create the top-level object and insert the nested rule.
        let mut top_level_attrs = CssAttributes::default();
        let selector = ":where(& > :not(:last-child))".to_string();
        top_level_attrs.insert_nested(selector, inner_attrs);

        top_level_attrs
    }
}