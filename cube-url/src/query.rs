use std::collections::{BTreeMap, btree_map};

use cube_core::{bytes::Scanner, error::Error};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Query {
    data: BTreeMap<String, String>,
}

impl Query {
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

impl IntoIterator for Query {
    type IntoIter = btree_map::IntoIter<String, String>;
    type Item = (String, String);

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json =
            serde_json::to_string(self).expect("[cube::http::uri::query] => failed to serialize");
        return write!(f, "{}", json);
    }
}

impl Query {
    pub fn parse(url: &str) -> Result<Self, Error> {
        let mut query = Query::new();
        let bytes = &mut Scanner::from(url);

        if !bytes.seek(b'?') {
            return Ok(query);
        }

        bytes.commit();

        while !bytes.is_eof() {
            let key = bytes.next_until(b'=').commit().to_string();
            bytes.fshift(1);

            let value = bytes.next_until(b'&').commit().to_string();
            bytes.fshift(1);
            query.set(&key, &value);
        }

        return Ok(query);
    }
}
