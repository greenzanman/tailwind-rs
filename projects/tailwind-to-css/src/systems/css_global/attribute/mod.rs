use super::*;

mod traits;

/// A css property is used to remove duplicates.
///
/// In principle, each css property will only appear once, and the one set later will override the previous one.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssAttributes {
    /// Normal css attributes with property and value.
    normal: ImportantMap,
    transforms: ImportantSet,
    backdrop_filter: ImportantSet,
    filter: ImportantSet,
    /// The key is the nested selector string, e.g., ":where(& > :not(:last-child))"
    /// The value is a CssAttributes for that nested rule.
    nested: BTreeMap<String, Box<CssAttributes>>, 
}

impl CssAttributes {
    /// # Arguments
    ///
    /// * `key`: css class
    /// * `value`: css property
    ///
    /// returns: [`CssAttribute`]
    #[track_caller]
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        match key.as_str() {
            "transform" => self.transforms.insert(value.into()),
            "backdrop-filter" => self.backdrop_filter.insert(value.into()),
            "filter" => self.filter.insert(value.into()),
            _ => self.normal.insert(key, value.into()),
        };
    }

    /// # Arguments
    ///
    /// * `items`:
    ///
    /// returns: ()
    pub fn extend<T>(&mut self, items: T)
    where
        T: IntoIterator<Item = (String, String)>,
    {
        for i in items {
            self.insert(i.0, i.1);
        }
    }

    /// Inserts a nested selector and its attributes.
    ///
    /// # Arguments
    /// * `selector`: The nested selector string (e.g., ":where(& > :not(:last-child))")
    /// * `attributes`: The CssAttributes to associate with the selector
    pub fn insert_nested<S>(&mut self, selector: S, attributes: CssAttributes)
    where
        S: Into<String>,
    {
        self.nested.insert(selector.into(), Box::new(attributes));
    }
}
