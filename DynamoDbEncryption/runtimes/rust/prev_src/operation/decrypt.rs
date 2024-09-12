// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Decrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Decrypt;
impl Decrypt {
    /// Creates a new `Decrypt`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::decrypt::DecryptRequest,
    ) -> ::std::result::Result<
        crate::operation::decrypt::DecryptResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::decrypt::_decrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Decrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::decrypt::_decrypt_output::from_dafny(
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

pub use crate::operation::decrypt::_decrypt_response::DecryptResponse;

pub use crate::operation::decrypt::_decrypt_request::DecryptRequest;

pub(crate) mod _decrypt_response;

pub(crate) mod _decrypt_request;

/// Builders
pub mod builders;
