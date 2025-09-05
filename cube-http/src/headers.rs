use std::collections::{BTreeMap, HashMap, btree_map};

use crate::Header;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Headers {
    data: BTreeMap<String, Header>,
}

impl Headers {
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

    pub fn get(&self, name: &str) -> Option<&Header> {
        return self.data.get(name);
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Header> {
        return self.data.get_mut(name);
    }

    pub fn set(&mut self, name: &str, value: &Header) {
        self.data.insert(name.to_string(), value.clone());
    }

    pub fn del(&mut self, name: &str) {
        self.data.remove(name);
    }
}

impl IntoIterator for Headers {
    type IntoIter = btree_map::IntoIter<String, Header>;
    type Item = (String, Header);

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

impl From<&HashMap<String, String>> for Headers {
    fn from(value: &HashMap<String, String>) -> Self {
        let mut headers = BTreeMap::<String, Header>::new();

        for (key, value) in value {
            headers.insert(key.clone(), Header::Raw(value.clone()));
        }

        return Self { data: headers };
    }
}

impl Into<HashMap<String, String>> for Headers {
    fn into(self) -> HashMap<String, String> {
        let mut headers = HashMap::<String, String>::new();

        for (key, value) in &self.data {
            headers.insert(key.clone(), value.to_string());
        }

        return headers;
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self.data)
            .expect("[cube::http::headers] => failed to serialize");
        return write!(f, "{}", json);
    }
}
