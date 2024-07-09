// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod DafnyLibraries {
    use dashmap::DashMap;
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub struct MutableMap<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> {
        map: DashMap<K, V>,
    }

    impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> MutableMap<K, V> {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::Object::new(MutableMap {
                map: DashMap::new(),
            })
        }
    }

    impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq>
        crate::DafnyLibraries::MutableMapTrait<K, V> for MutableMap<K, V>
    {
        fn content(&self) -> ::dafny_runtime::Map<K, V> {
            let mut new_map = HashMap::new();
            for entry in self.map.iter() {
                new_map.insert(entry.key().clone(), entry.value().clone());
            }
            dafny_runtime::Map::from_hashmap_owned(new_map)
        }
        fn Put(&mut self, k: &K, v: &V) -> () {
            self.map.insert(k.clone(), v.clone());
        }
        fn Keys(&self) -> ::dafny_runtime::Set<K> {
            let mut new_set = HashSet::new();
            for entry in self.map.iter() {
                new_set.insert(entry.key().clone());
            }
            dafny_runtime::Set::from_hashset_owned(new_set)
        }
        fn HasKey(&self, k: &K) -> bool {
            self.map.contains_key(k)
        }
        fn Values(&self) -> ::dafny_runtime::Set<V> {
            let mut new_set = HashSet::new();
            for entry in self.map.iter() {
                new_set.insert(entry.value().clone());
            }
            dafny_runtime::Set::from_hashset_owned(new_set)
        }
        fn Items(&self) -> ::dafny_runtime::Set<(K, V)> {
            let mut new_set = HashSet::new();
            for entry in self.map.iter() {
                new_set.insert((entry.key().clone(), entry.value().clone()));
            }
            dafny_runtime::Set::from_hashset_owned(new_set)
        }
        fn Select(&self, k: &K) -> V {
            self.map.get(k).unwrap().clone()
        }
        fn Remove(&mut self, k: &K) -> () {
            self.map.remove(k);
        }
        fn Size(&self) -> ::dafny_runtime::DafnyInt {
            self.map.len().into()
        }
    }

    pub mod FileIO {
        pub fn INTERNAL_ReadBytesFromFile(
            _file: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> (
            bool,
            ::dafny_runtime::Sequence<u8>,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) {
            todo!("r#_Dafny_dFileIO::r#_INTERNAL_ReadBytesFromFile not implemented");
        }
        pub fn INTERNAL_WriteBytesToFile(
            _path: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            _bytes: &::dafny_runtime::Sequence<u8>,
        ) -> (
            bool,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) {
            todo!("r#_Dafny_dFileIO::(path: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, bytes: &::dafny_runtime::Sequence<u8>) not implemented");
        }
    }
}
