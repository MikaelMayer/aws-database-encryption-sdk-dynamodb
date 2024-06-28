// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

impl crate::UUID::_default {
  pub fn ToByteArray(_bytes: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>)
    -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
  {
    todo!("UUID::ToByteArray not implemented");
  }

  pub fn FromByteArray(_bytes: &::dafny_runtime::Sequence<u8>)
    -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
  {
    todo!("UUID::FromByteArray not implemented");
  }

  pub fn GenerateUUID() -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>> {
    todo!("UUID::GenerateUUID not implemented");
  }
}
