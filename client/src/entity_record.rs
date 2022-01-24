use std::collections::HashMap;

use naia_shared::{LocalComponentKey, ProtocolKindType};

#[derive(Debug)]
pub struct EntityRecord<K: ProtocolKindType> {
    kind_to_key_map: HashMap<K, LocalComponentKey>,
    key_to_kind_map: HashMap<LocalComponentKey, K>,
}

impl<K: ProtocolKindType> EntityRecord<K> {
    pub fn new() -> Self {
        EntityRecord {
            kind_to_key_map: HashMap::new(),
            key_to_kind_map: HashMap::new(),
        }
    }

    // Components / Kinds //

    pub fn get_kind_from_key(&self, component_key: &LocalComponentKey) -> Option<&K> {
        return self.key_to_kind_map.get(component_key);
    }

    pub fn insert_component(&mut self, key: &LocalComponentKey, kind: &K) {
        self.kind_to_key_map.insert(*kind, *key);
        self.key_to_kind_map.insert(*key, *kind);
    }

    pub fn remove_component(&mut self, key: &LocalComponentKey) -> Option<K> {
        if let Some(kind) = self.key_to_kind_map.remove(key) {
            self.kind_to_key_map.remove(&kind);
            return Some(kind);
        }
        return None;
    }

    pub fn get_component_keys(&self) -> Vec<LocalComponentKey> {
        let mut output = Vec::<LocalComponentKey>::new();
        for (key, _) in self.key_to_kind_map.iter() {
            output.push(*key);
        }
        return output;
    }
}
