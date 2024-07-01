// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod SortedSets {
  use crate::*;
  pub struct _default {}
  impl _default {
    pub fn SetToSequence<T: ::dafny_runtime::DafnyTypeEq>(_elems: &::dafny_runtime::Set<T>) -> ::dafny_runtime::Sequence<T> {
      todo!("SortedSets::SetToSequence not implemented");
    }
    pub fn SetToOrderedSequence2<T: ::dafny_runtime::DafnyTypeEq>(_elems: &::dafny_runtime::Set<::dafny_runtime::Sequence<T>>, less: &Rc<dyn Fn(&T, &T) -> bool>) -> ::dafny_runtime::Sequence<T> {
      todo!("SortedSets::SetToOrderedSequence2 not implemented");
    }
  }  
}

