// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DeriveSharedSecret`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeriveSharedSecret;
impl DeriveSharedSecret {
    /// Creates a new `DeriveSharedSecret`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::derive_shared_secret::DeriveSharedSecretRequest,
    ) -> ::std::result::Result<
        crate::operation::derive_shared_secret::DeriveSharedSecretResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::derive_shared_secret::_derive_shared_secret_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DeriveSharedSecret(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::derive_shared_secret::_derive_shared_secret_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::derive_shared_secret::_derive_shared_secret_response::DeriveSharedSecretResponse;

pub use crate::operation::derive_shared_secret::_derive_shared_secret_request::DeriveSharedSecretRequest;

pub(crate) mod _derive_shared_secret_response;

pub(crate) mod _derive_shared_secret_request;

/// Builders
pub mod builders;
