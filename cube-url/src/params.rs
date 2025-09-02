use std::collections::{BTreeMap, btree_map};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Params {
    data: BTreeMap<String, String>,
}

impl Params {
    pub fn new() -> Self {
        return Self {
            data: BTreeMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn has(&self, name: &str) -> bool {
        return self.data.contains_key(name);
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        return self.data.get(name);
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut String> {
        return self.data.get_mut(name);
    }

    pub fn set(&mut self, name: &str, value: &String) {
        self.data.insert(name.to_string(), value.clone());
    }

    pub fn del(&mut self, name: &str) {
        self.data.remove(name);
    }
}

impl IntoIterator for Params {
    type IntoIter = btree_map::IntoIter<String, String>;
    type Item = (String, String);

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json =
            serde_json::to_string(self).expect("[cube::http::uri::query] => failed to serialize");
        return write!(f, "{}", json);
    }
}
