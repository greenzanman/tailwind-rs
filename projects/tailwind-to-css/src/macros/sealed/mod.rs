macro_rules! keyword_instance {
    // ARM 0: Handles types that have special case(s) for input -> output_kword mappings.
    //  - example: keyword_instance!(TailwindDisplay => "display", { "hidden" => "none" });
    //  - $type example: TailwindDisplay
    //  - $attr_property example: "display"
    ($type:ty => $attr_property:literal, { $($input_kword:literal => $output_kword:literal),* $(,)? }) => {
        impl<T> From<T> for $type
        where
            T: Into<String>,
        {
            fn from(input_kword: T) -> Self {
                let s: String = input_kword.into();
                // Get the CSS value associated with the input_kword keyword.
                // Also set that CSS value as the new keyword.
                // 1. this will normalize the keyword and simplify the setting of (property:value);
                let attr_value = match s.as_str() {
                    $(
                        $input_kword => $output_kword.to_string(),
                    )*
                    // If no match, use the original string
                    _ => s,
                };
                Self { kind: StandardValue::from(attr_value) }
            }
        }

        // Set a css declaration (property:value;) for this keyword (StandardValue)
        impl TailwindInstance for $type {
            fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
                css_attributes! {
                    $attr_property => self.kind.get_properties()
                }
            }
        }

        // Generate the mapping function from the provided key-values
        impl $type {
            #[doc = "Maps keyword aliases to their canonical CSS values.\n\n"]
            #[doc = "This function ensures that shorthands used in Tailwind classes (like `justify-between`)"]
            #[doc = "and direct instantiations are normalized to a single, consistent value before being stored."]
            #[doc = "For example, `between` is mapped to `space-between`.\n\n"]
            #[doc = "It should be used internally by both the `parse` method and the `From<T>` implementation, "]
            #[doc = "making this the single source of truth for keyword normalization.\n\n"]
            #[doc = "## Examples\n\n"]
            #[doc = "```"]
            #[doc = "// The type name here would be whatever type the macro is implementing,"]
            #[doc = "// for instance, `TailwindJustifyContent`."]
            #[doc = "assert_eq!(Self::map_keyword(\"between\"), \"space-between\");"]
            #[doc = "assert_eq!(Self::map_keyword(\"start\"), \"flex-start\");"]
            #[doc = "// Keywords without an alias are returned as-is."]
            #[doc = "assert_eq!(Self::map_keyword(\"center\"), \"center\");"]
            #[doc = "```"]
            pub fn map_keyword(s: &str) -> &str {
                match s {
                    $(
                        $input_kword => $output_kword,
                    )*
                    // For any other valid keyword, return it as is.
                    other => other,
                }
            }
        }
    };

    // ARM 1: The original macro for types without special mappings.
    // NOTE: This comes second, bc macros match the most specific rule first
    //   - example: keyword_instance!(TailwindDisplay => "display");
    ($type:ty => $attr_property:literal) => {
        impl<T> From<T> for $type
        where
            T: Into<String>,
        {
            fn from(input_kword: T) -> Self {
                Self { kind: StandardValue::from(input_kword.into()) }
            }
        }
        // Same implementation as ARM 0
        impl TailwindInstance for $type {
            fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
                css_attributes! {
                    $attr_property => self.kind.get_properties()
                }
            }
        }
    };
}

macro_rules! color_instance {
    ($type:ty) => {
        impl<T> From<T> for $type
        where
            T: Into<TailwindColor>,
        {
            fn from(color: T) -> Self {
                Self { color: color.into() }
            }
        }
        impl $type {
            ///
            pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse(input, arbitrary)? })
            }
            ///
            pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
                Ok(Self { color: TailwindColor::parse_arbitrary(arbitrary)? })
            }
        }
    };
}
macro_rules! css_insert {
    // not work!!!
    // ($css:expr, "transform", $v:expr) => {
    //     $css.transform($v.to_string());
    // };
    // ($css:expr, "filter", $v:expr) => {
    //     $css.filter($v.to_string());
    // };
    // ($css:expr, "backdrop-filter", $v:expr) => {
    //     $css.backdrop_filter($v.to_string());
    // };
    ($css:expr, $k:expr,  $v:expr) => {
        $css.insert($k.to_string(), $v.to_string());
    };
}

pub(crate) use color_instance;
pub(crate) use css_insert;
pub(crate) use keyword_instance;
