// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

pub mod AesKdfCtr {
  pub struct _default {}

  impl _default {
    pub fn AesKdfCtrStream(nonce: &::dafny_runtime::Sequence<u8>, key: &::dafny_runtime::Sequence<u8>, length: u32) -> ::std::rc::Rc<super::Wrappers::Result<::dafny_runtime::Sequence<u8>, super::r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::OpaqueError>> {
      todo!("AesKdfCtrStream not implemented")
    }
  }
}