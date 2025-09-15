use super::*;

mod builtin;

///
#[derive(Clone, Debug, Default)]
pub struct PaletteSystem {
    inner: HashMap<String, Palette>,
    keywords: HashMap<String, Srgb>,
}

impl PaletteSystem {
    pub fn try_get_color(&self, name: &str, weight: u32) -> Result<Srgb> {
        match self.inner.get(name) {
            Some(p) => p.get_color(weight),
            None => syntax_error!("no such palette"),
        }
    }

    pub fn try_get_keyword_color(&self, name: &str) -> Result<&Srgb> {
        match self.keywords.get(name) {
            Some(color) => Ok(color),
            None => syntax_error!("no such keyword in palette '{}'", name),
        }
    }

    #[inline]
    pub fn register(&mut self, name: String, colors: Palette) -> Option<Palette> {
        self.inner.insert(name, colors)
    }

    /// Registers a keyword from any valid CSS color string (e.g., "#FFF", "rgba(...)").
    /// - uses the `css_color` crate for parsing.
    #[inline]
    pub fn register_keyword(&mut self, name: String, color_string: &str) -> Result<()> {
        let color = TailwindArbitrary::from(color_string).as_color()?;
        self.keywords.insert(name, color);
        Ok(())
    }
}
