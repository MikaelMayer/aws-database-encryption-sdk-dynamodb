// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ConnectCustomKeyStore`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ConnectCustomKeyStore;
impl ConnectCustomKeyStore {
    /// Creates a new `ConnectCustomKeyStore`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::connect_custom_key_store::ConnectCustomKeyStoreRequest,
    ) -> ::std::result::Result<
        crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::connect_custom_key_store::_connect_custom_key_store_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ConnectCustomKeyStore(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::connect_custom_key_store::_connect_custom_key_store_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::connect_custom_key_store::_connect_custom_key_store_response::ConnectCustomKeyStoreResponse;

pub use crate::operation::connect_custom_key_store::_connect_custom_key_store_request::ConnectCustomKeyStoreRequest;

pub(crate) mod _connect_custom_key_store_response;

pub(crate) mod _connect_custom_key_store_request;

/// Builders
pub mod builders;
