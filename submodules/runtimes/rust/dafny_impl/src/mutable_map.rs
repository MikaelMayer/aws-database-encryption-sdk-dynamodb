// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::ImplementationFromDafny::*;

pub mod DafnyLibraries {
  use crate::ImplementationFromDafny::*;
  pub struct MutableMap<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> {
    _phantom_type_param_0: ::std::marker::PhantomData<K>,
    _phantom_type_param_1: ::std::marker::PhantomData<V>
  }

  impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> MutableMap<K, V> {
  }

  impl<K: ::dafny_runtime::DafnyTypeEq, V: ::dafny_runtime::DafnyTypeEq> DafnyLibraries::MutableMapTrait<K, V>
    for MutableMap<K, V> {
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
}
