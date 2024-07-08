// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
pub mod DafnyLibraries {
    pub struct MutableMap<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> {
        _phantom_type_param_0: ::std::marker::PhantomData<K>,
        _phantom_type_param_1: ::std::marker::PhantomData<V>,
    }

    impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> MutableMap<K, V> {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            // SAFETY: The Rc has not been shared before
            unsafe {
                ::dafny_runtime::Object::from_rc(::std::rc::Rc::new(MutableMap {
                    _phantom_type_param_0: todo!(),
                    _phantom_type_param_1: todo!(),
                }))
            }
        }
    }

    impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq>
        crate::DafnyLibraries::MutableMapTrait<K, V> for MutableMap<K, V>
    {
        fn content(&self) -> ::dafny_runtime::Map<K, V> {
            todo!("MutableMap::content not implemented");
        }
        fn Put(&mut self, _k: &K, _v: &V) -> () {
            todo!("MutableMap::Put not implemented");
        }
        fn Keys(&self) -> ::dafny_runtime::Set<K> {
            todo!("MutableMap::Keys not implemented");
        }
        fn HasKey(&self, _k: &K) -> bool {
            todo!("MutableMap::HasKey not implemented");
        }
        fn Values(&self) -> ::dafny_runtime::Set<V> {
            todo!("MutableMap::Values not implemented");
        }
        fn Items(&self) -> ::dafny_runtime::Set<(K, V)> {
            todo!("MutableMap::Items not implemented");
        }
        fn Select(&self, _k: &K) -> V {
            todo!("MutableMap::Select not implemented");
        }
        fn Remove(&mut self, _k: &K) -> () {
            todo!("MutableMap::Remove not implemented");
        }
        fn Size(&self) -> ::dafny_runtime::DafnyInt {
            todo!("MutableMap::Size not implemented");
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
