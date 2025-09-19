use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::{CssAttributes, TailwindBuilder};

pub mod instance;

#[allow(unused_variables)]
pub trait TailwindInstance: Display {
    /// used to deduplication and marking
    #[inline]
    fn id(&self) -> String {
        normalize_html_class_name(&self.to_string())
    }
    /// Is this instance inlineable within html style="" attribute?
    /// - (no nested rules, no pseudo-classes, etc)
    fn inlineable(&self) -> bool {
        true
    }
    /// New tailwind instance
    fn boxed(self) -> Box<dyn TailwindInstance>
    where
        Self: Sized,
        Self: 'static,
    {
        Box::new(self)
    }
    /// Custom selector name
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        format!(".{}", self.id())
    }
    /// Attributes in css, representing contained CSS property-value(s)
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes;
    /// Additional css in bundle
    fn additional(&self, ctx: &TailwindBuilder) -> String {
        String::new()
    }
}

/// Normalize classname to be used as a valid HTML classname.
/// - Escapes non-alphanumeric characters with a backslash (`\`).
/// - Replaces spaces with underscores (`_`).
/// 
/// Same as normalizing classname as CSS selector, but without escaping non-alphanumeric characters
fn normalize_html_class_name(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        match c {
            ' ' => out.push('_'),
            r @ ('-' | '_') => out.push(r),
            a if a.is_alphanumeric() => out.push(a),
            _ => out.push(c),
        }
    }
    out
}