// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ReplicateKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ReplicateKey;
impl ReplicateKey {
    /// Creates a new `ReplicateKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::replicate_key::ReplicateKeyRequest,
    ) -> ::std::result::Result<
        crate::operation::replicate_key::ReplicateKeyResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::replicate_key::_replicate_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ReplicateKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::replicate_key::_replicate_key_output::from_dafny(
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

pub use crate::operation::replicate_key::_replicate_key_response::ReplicateKeyResponse;

pub use crate::operation::replicate_key::_replicate_key_request::ReplicateKeyRequest;

pub(crate) mod _replicate_key_response;

pub(crate) mod _replicate_key_request;

/// Builders
pub mod builders;
