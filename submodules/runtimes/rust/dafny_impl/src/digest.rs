// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod ExternDigest {
  use crate::*;
  pub struct _default {}
  impl _default {
    #[allow(non_snake_case)]
    pub fn Digest(
      _digestAlgorithm: &::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm>,
      _message: &::dafny_runtime::Sequence<u8>
    ) -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, ::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::Error>>> {
      todo!("Digest not implemented");
    }
  }
}
