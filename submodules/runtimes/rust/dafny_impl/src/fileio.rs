// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::ImplementationFromDafny::*;

pub mod r#_DafnyLibraries_dFileIO {

  pub struct _default {}
  
  impl _default {
    pub fn r#_INTERNAL_ReadBytesFromFile(_file: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>)
      -> (bool, ::dafny_runtime::Sequence<u8>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>)
    {
      todo!("r#_Dafny_dFileIO::r#_INTERNAL_ReadBytesFromFile not implemented");
    }
    pub fn r#_INTERNAL_WriteBytesToFile(_path: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, _bytes: &::dafny_runtime::Sequence<u8>)
      -> (bool, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>)
    {
      todo!("r#_Dafny_dFileIO::(path: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, bytes: &::dafny_runtime::Sequence<u8>) not implemented");
    }
  }
}
