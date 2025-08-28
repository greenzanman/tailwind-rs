use super::*;

mod traits;

/// Used to represent CSS properties that have keyword values.
#[derive(Debug, Clone)]
pub enum StandardValue {
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

impl StandardValue {
    pub fn parser(
        id: &'static str,
        check_valid: &'static impl Fn(&str) -> bool,
    ) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => Self::parse_keyword(pattern, id, check_valid),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    pub fn parse_keyword(pattern: &[&str], id: &str, checker: &'static impl Fn(&str) -> bool) -> Result<Self> {
        let keyword = pattern.join("-");
        if cfg!(feature = "compile_time") && !checker(&keyword) {
            return syntax_error!("{} does not a valid value of {}", keyword, id);
        }
        Ok(Self::Keyword(keyword))
    }
    pub fn get_properties(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    
    /// A helper for writing CSS classnames for `StandardValue`s (Tailwind-style utilities 
    /// that represent the CSS properties with keyword values).
    ///
    /// This function handles the logic for formatting `Keyword` and `Arbitrary` values
    /// based on a set of instructions provided by a transformer closure.
    ///
    /// ## Arguments
    /// * `class_prefix`: The static part of the classname that precedes the value (e.g., `"isolation-"`).
    /// * `transformer`: A closure that takes an input keyword and returns a `KeywordClassFormat`
    ///   variant, which dictates how the final classname will be written.
    /// 
    /// ## Example
    /// ```
    /// impl Display for TailwindIsolation {
    /// fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    ///     self.kind.write_class(f, "isolation-", |s| match s {
    ///         // Special Case: The output is just the keyword itself, without the "isolation-" prefix.
    ///         "isolate" => KeywordClassFormat::CustomClassname("isolate"),
    ///
    ///         // General Rule: The output requires a prefix.
    ///         keyword if TailwindIsolation::check_valid(keyword) => KeywordClassFormat::AddAsSuffix,
    ///
    ///         // Anything else is invalid.
    ///         _ => KeywordClassFormat::InvalidKeyword,
    ///     })
    /// }
    /// ```
    pub fn write_class(
        &self,
        fmt: &mut Formatter,
        class_prefix: &str,
        transformer: fn(&str) -> KeywordClassFormat,
    ) -> std::fmt::Result {
        match self {
            StandardValue::Keyword(s) => {
                match transformer(s) {
                    // Custom: {}
                    KeywordClassFormat::CustomClassname(value) => write!(fmt, "{}", value),

                    // Non-custom: isolation-{}
                    KeywordClassFormat::AddAsSuffixCustom(value) => write!(fmt, "{}{}", class_prefix, value),
                    KeywordClassFormat::AddAsSuffix => write!(fmt, "{}{}", class_prefix, s),
                    KeywordClassFormat::InvalidKeyword => Err(std::fmt::Error).into(),
                }
            }
            StandardValue::Arbitrary(s) => write!(fmt, "{}[{}]", class_prefix, s),
        }
    }
}

/// Describes an instruction for the `write_class` function on how to format a CSS classname.
#[derive(Debug, Clone)]
pub enum KeywordClassFormat<'a> {
    /// Writes a completely custom classname, ignoring the `class_prefix`.
    ///
    /// # Example
    /// Given a `class_prefix` of `"prefix-"`, returning `CustomClassname("isolate")`
    /// will result in the final classname: `isolate`.
    CustomClassname(&'a str),

    /// Appends a specific, transformed string to the `class_prefix`.
    ///
    /// # Example
    /// Given a `class_prefix` of `"prefix-"` and an input of `"column-dense"`, returning
    /// `AddAsSuffixCustom("col-dense")` results in the final classname: `prefix-col-dense`.
    AddAsSuffixCustom(&'a str),

    /// Appends the original, untransformed keyword to the `class_prefix`.
    ///
    /// # Example
    /// Given a `class_prefix` of `"prefix-"` and an input of `"auto"`, returning
    /// `AddAsSuffix` results in the final classname: `prefix-auto`.
    AddAsSuffix,

    /// Indicates the keyword is invalid, causing `write_class` to return a formatting error.
    InvalidKeyword,
}