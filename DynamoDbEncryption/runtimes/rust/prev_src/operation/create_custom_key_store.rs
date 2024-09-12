// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateCustomKeyStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateCustomKeyStore;
impl CreateCustomKeyStore {
    /// Creates a new `CreateCustomKeyStore`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::create_custom_key_store::CreateCustomKeyStoreRequest,
    ) -> ::std::result::Result<
        crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::create_custom_key_store::_create_custom_key_store_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateCustomKeyStore(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::create_custom_key_store::_create_custom_key_store_output::from_dafny(
                    inner_result.value().clone(),
                ),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::create_custom_key_store::_create_custom_key_store_response::CreateCustomKeyStoreResponse;

pub use crate::operation::create_custom_key_store::_create_custom_key_store_request::CreateCustomKeyStoreRequest;

pub(crate) mod _create_custom_key_store_response;

pub(crate) mod _create_custom_key_store_request;

/// Builders
pub mod builders;
