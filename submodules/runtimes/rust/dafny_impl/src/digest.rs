// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

impl crate::ExternDigest::_default {
  #[allow(non_snake_case)]
  pub fn Digest(
    _digestAlgorithm: &::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::DigestAlgorithm>,
    _message: &::dafny_runtime::Sequence<u8>
  ) -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
    todo!("Digest not implemented");
  }
}
