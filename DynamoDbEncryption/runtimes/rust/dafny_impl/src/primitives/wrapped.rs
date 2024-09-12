// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod client;

impl crate::software::amazon::cryptography::primitives::internaldafny::wrapped::_default {
  pub fn WrappedAtomicPrimitives(config: &::std::rc::Rc<
      crate::software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig,
  >) -> ::std::rc::Rc<crate::Wrappers::Result<
          ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::primitives::internaldafny::types::IAwsCryptographicPrimitivesClient>,
          ::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>
  >>{
      crate::primitives::wrapped::client::Client::from_conf(config)
  }
}
