// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use std::sync::LazyLock;

pub struct Client {
    wrapped: crate::material_providers::client::Client
}

/// A runtime for executing operations on the asynchronous client in a blocking manner.
/// Necessary because Dafny only generates synchronous code.
static dafny_tokio_runtime: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()
          .unwrap()
});

impl dafny_runtime::UpcastObject<dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient);
}

impl dafny_runtime::UpcastObject<dyn std::any::Any> for Client {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

impl Client {
  pub fn from_conf(config: &::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KeyStoreConfig,
  >) ->
::std::rc::Rc<crate::Wrappers::Result<
  ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient>,
  ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>
>> {
    let result = crate::material_providers::client::Client::from_conf(
      crate::material_providers::conversions::key_store_config::_key_store_config::from_dafny(
          config.clone(),
      ),
    );
    match result {
      Ok(client) =>  {
        let wrap = crate::material_providers::wrapped::client::Client {
          wrapped: client
        };
        std::rc::Rc::new(
          crate::Wrappers::Result::Success {
            value: ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(wrap))
          }
        )
      },
      Err(error) => crate::material_providers::conversions::error::to_opaque_error_result(error)
    }
  }
}

impl crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient for Client {
    fn CreateKey(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::create_key::_create_key_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::create_key::CreateKey::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::create_key::_create_key_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn CreateKeyStore(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::CreateKeyStoreOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::create_key_store::_create_key_store_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::create_key_store::CreateKeyStore::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::create_key_store::_create_key_store_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn GetActiveBranchKey(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetActiveBranchKeyInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetActiveBranchKeyOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::get_active_branch_key::_get_active_branch_key_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::get_active_branch_key::GetActiveBranchKey::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::get_active_branch_key::_get_active_branch_key_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn GetBeaconKey(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBeaconKeyInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBeaconKeyOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::get_beacon_key::_get_beacon_key_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::get_beacon_key::GetBeaconKey::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::get_beacon_key::_get_beacon_key_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn GetBranchKeyVersion(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBranchKeyVersionOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::get_branch_key_version::_get_branch_key_version_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersion::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::get_branch_key_version::_get_branch_key_version_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn GetKeyStoreInfo(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Unit>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetKeyStoreInfoOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::get_key_store_info::_get_key_store_info_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::get_key_store_info::GetKeyStoreInfo::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::get_key_store_info::_get_key_store_info_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn VersionKey(
        &mut self,
        input: &::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::VersionKeyInput>,
    ) -> std::rc::Rc<
        crate::Wrappers::Result<
            ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::VersionKeyOutput>,
            std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::material_providers::conversions::version_key::_version_key_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::material_providers::operation::version_key::VersionKey::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Failure {
                    error: crate::material_providers::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::Wrappers::Result::Success {
                    value: crate::material_providers::conversions::version_key::_version_key_output::to_dafny(inner_result),
                },
            ),
        }
    }
}
