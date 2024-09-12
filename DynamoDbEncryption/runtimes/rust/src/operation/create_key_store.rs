// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateKeyStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateKeyStore;
impl CreateKeyStore {
    /// Creates a new `CreateKeyStore`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::create_key_store::CreateKeyStoreInput,
    ) -> ::std::result::Result<
        crate::operation::create_key_store::CreateKeyStoreOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::create_key_store::_create_key_store_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateKeyStore(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::create_key_store::_create_key_store_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::create_key_store::_create_key_store_output::CreateKeyStoreOutput;

pub use crate::operation::create_key_store::_create_key_store_input::CreateKeyStoreInput;

pub(crate) mod _create_key_store_output;

pub(crate) mod _create_key_store_input;

/// Builders
pub mod builders;
