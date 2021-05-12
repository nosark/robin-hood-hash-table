
mod rh_hash_table {
    use std::collections::hash_map::{RandomState, DefaultHasher};
    use std::hash::{BuildHasher, Hasher};

    struct KeyValuePair<K, V> {
        key: K,
        value:V,
    }

    impl <K, V> KeyValuePair<K, V> {
        fn new(key: K, value: V) -> Self {
            Self { key, value }
        }
    }
    #[allow(dead_code)]
    struct RobinHoodHashTable<KeyValuePair> {
        num_entries:i64,
        max_load_factor:f64,
        table: Vec::<KeyValuePair>,
        hasher: DefaultHasher,
    }

    impl<K, V> RobinHoodHashTable<KeyValuePair<K, V>> {
        #[allow(dead_code)]
        fn new(max_load: f64) -> Box<Self> {
            let hasher_state = RandomState::new();
            let default_hasher = hasher_state.build_hasher();
            Box::new(Self {
                num_entries: 0,
                max_load_factor: max_load,
                table: Vec::<KeyValuePair<K, V>>::new(),
                hasher: default_hasher }
            )
        }

        fn insert<K, V>(key: K, value: V) {
            unimplemented!()
        }

        fn remove<K, V>(key: K, value: V) {
            unimplemented!()
        }

        fn lookup<K, V>(key: K) -> V {
            unimplemented!()
        }
    }
}