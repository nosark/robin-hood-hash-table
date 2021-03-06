pub mod rh_hash_table {
    use std::collections::hash_map::{DefaultHasher, Keys, RandomState};
    use std::fmt::Display;
    use std::hash::{BuildHasher, Hash, Hasher};
    use std::ops::Deref;

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub struct KeyValuePair<K, V> {
        pub key: K,
        value: V,
        pub probing_sequence_length: i64,
    }

    impl<K: Hash + Clone + Eq + Copy, V: Clone + Copy> KeyValuePair<K, V> {
        pub fn new(key: K, value: V, psl: i64) -> Self {
            Self {
                key,
                value,
                probing_sequence_length: psl,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct RobinHoodHashTable<KeyValuePair> {
        pub capacity: usize,
        num_entries: i64,
        max_load_factor: f64,
        pub table: Vec<Option<KeyValuePair>>,
        pub hasher_state: RandomState,
    }

    impl<K: Hash + Display + Clone + Eq, V: Display + Clone + Eq>
        RobinHoodHashTable<KeyValuePair<K, V>>
    {
        /// When we create a new hash table we must define the capacity for later resizing
        /// Currently we create a hasher using the default SipHash implementation.
        pub fn new(max_load: f64, capacity: usize) -> Box<Self> {
            let hasher_state = RandomState::new();
            Box::new(Self {
                capacity,
                num_entries: 0,
                max_load_factor: max_load,
                table: vec![None; capacity],
                hasher_state,
            })
        }

        pub fn insert(&mut self, key: K, value: V) {
            let mut key_value = KeyValuePair {
                key,
                value,
                probing_sequence_length: 0,
            };
            let mut hasher = self.hasher_state.build_hasher();
            key_value.key.hash(&mut hasher);
            let mut hash_id = hasher.finish() as usize % self.capacity;
            loop {
                let mut bucket = self.table[hash_id].clone();
                match bucket {
                    Some(..) => {
                        if bucket.as_ref().unwrap().probing_sequence_length
                            > key_value.probing_sequence_length
                        {
                            let temp = bucket.clone().unwrap();
                            bucket = Some(key_value.clone());
                            key_value = temp;
                        }

                        key_value.probing_sequence_length += 1;
                        hash_id += 1;
                        if hash_id >= self.capacity {
                            hash_id = 0;
                        }
                    }

                    None => {
                        self.table[hash_id] = Some(key_value);
                        self.num_entries += 1;
                        break;
                    }
                    _ => panic!("Something went wrong while inserting element!!!"),
                }
            }

            let current_load: f64 = self.num_entries as f64 / self.capacity as f64;
            if current_load >= self.max_load_factor {
                self.build_resized_table();
            }
        }

        pub fn build_resized_table(&mut self) {
            let resized_table: Vec<Option<KeyValuePair<K, V>>> = vec![None; self.capacity * 2];
            let temp_table = self.table.clone();
            self.table = resized_table;
            self.capacity *= 2;

            for i in 0..temp_table.len() {
                if !temp_table[i].is_none() {
                    let new_entry = temp_table[i].as_ref().unwrap().clone();
                    self.insert(new_entry.key, new_entry.value);
                }
            }
        }

        pub fn remove(&mut self, key: K) -> bool {
            let mut hasher = self.hasher_state.build_hasher();
            key.hash(&mut hasher);
            let mut hash_id = hasher.finish() as usize % self.capacity;
            loop {
                let mut bucket = self.table[hash_id].clone();
                match bucket {
                    Some(..) => {
                        if bucket.unwrap().key == key {
                            self.table[hash_id] = None;
                            return true;
                        }

                        hash_id += 1;
                        if hash_id >= self.capacity {
                            hash_id = 0;
                        }
                    }
                    None => {
                        break;
                    }
                    _ => {
                        panic!(
                            "something went wrong while trying to delete {} from table!!!",
                            key
                        );
                    }
                }
            }
            false
        }

        pub fn contains(&mut self, key: K) -> bool {
            let mut probing_sequence_len = 0;
            let mut hasher = self.hasher_state.build_hasher();
            key.hash(&mut hasher);
            let mut hash_id = hasher.finish() as usize % self.capacity;
            loop {
                let mut bucket = self.table[hash_id].as_ref();
                match bucket {
                    Some(..) => {
                        if probing_sequence_len > bucket.unwrap().probing_sequence_length {
                            return false;
                        }
                        if bucket.unwrap().key == key {
                            return true;
                        }
                        probing_sequence_len += 1;
                        hash_id += 1;
                        if hash_id >= self.capacity {
                            hash_id = 0;
                        }
                    }

                    None => {
                        break;
                    }

                    _ => panic!("something went wrong while searching for {} in table", key),
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rh_hash_table::{KeyValuePair, RobinHoodHashTable};
    use std::hash::{Hash, Hasher};
    #[test]
    fn insert_test_for_all_cases() {
        let mut rht = RobinHoodHashTable::new(0.9, 3);
        rht.insert(String::from("pineapple"), 1);
        assert_eq!(rht.contains(String::from("pineapple")), true);

        rht.insert(String::from("carrot"), 2);
        rht.insert(String::from("cucumber"), 3);

        assert_eq!(rht.contains(String::from("carrot")), true);
        assert_eq!(rht.contains(String::from("cucumber")), true);
    }
    #[test]
    fn contains_test_for_search_key_that_exists() {
        let mut rht = RobinHoodHashTable::new(0.9, 3);
        rht.insert("pine tree", 1);
        assert_eq!(rht.contains("pine tree"), true);
    }

    #[test]
    fn contains_test_for_search_key_that_doesnt_exist() {
        let mut rht = RobinHoodHashTable::<KeyValuePair<&str, i64>>::new(0.9, 3);
        assert_eq!(rht.contains("pine tree"), false);
    }

    #[test]
    fn remove_key_from_table() {
        let mut rht = RobinHoodHashTable::new(0.9, 3);
        rht.insert("pine tree", 1);
        assert_eq!(rht.contains("pine tree"), true);

        rht.remove("pine tree");

        assert_eq!(rht.contains("pine tree"), false);

        assert_eq!(rht.remove("pine tree"), false);
    }
}
