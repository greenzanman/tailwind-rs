use super::*;

impl From<Srgb> for TailwindColor {
    fn from(c: Srgb) -> Self {
        Self::RGB(c)
    }
}

impl From<&str> for TailwindColor {
    fn from(s: &str) -> Self {
        Self::Keyword { name: s.to_string(), alpha: None }
    }
}

impl From<String> for TailwindColor {
    fn from(s: String) -> Self {
        Self::Keyword { name: s, alpha: None }
    }
}
