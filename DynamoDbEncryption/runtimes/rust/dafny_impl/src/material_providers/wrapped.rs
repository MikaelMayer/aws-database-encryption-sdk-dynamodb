// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod client;

impl crate::r#software::amazon::cryptography::keystore::internaldafny::wrapped::_default {
  pub fn WrappedKeyStore(config: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig,
  >) -> ::std::rc::Rc<crate::Wrappers::Result<
          ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient>,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>
  >>{
      crate::material_providers::wrapped::client::Client::from_conf(config)
  }
}
