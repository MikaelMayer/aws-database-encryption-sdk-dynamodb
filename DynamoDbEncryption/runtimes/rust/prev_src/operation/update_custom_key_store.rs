// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateCustomKeyStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateCustomKeyStore;
impl UpdateCustomKeyStore {
    /// Creates a new `UpdateCustomKeyStore`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_custom_key_store::UpdateCustomKeyStoreRequest,
    ) -> ::std::result::Result<
        crate::operation::update_custom_key_store::UpdateCustomKeyStoreResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_custom_key_store::_update_custom_key_store_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateCustomKeyStore(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_custom_key_store::_update_custom_key_store_output::from_dafny(
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

pub use crate::operation::update_custom_key_store::_update_custom_key_store_response::UpdateCustomKeyStoreResponse;

pub use crate::operation::update_custom_key_store::_update_custom_key_store_request::UpdateCustomKeyStoreRequest;

pub(crate) mod _update_custom_key_store_response;

pub(crate) mod _update_custom_key_store_request;

/// Builders
pub mod builders;
