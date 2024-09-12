// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ReEncrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ReEncrypt;
impl ReEncrypt {
    /// Creates a new `ReEncrypt`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::re_encrypt::ReEncryptRequest,
    ) -> ::std::result::Result<
        crate::operation::re_encrypt::ReEncryptResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::re_encrypt::_re_encrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ReEncrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::re_encrypt::_re_encrypt_output::from_dafny(
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

pub use crate::operation::re_encrypt::_re_encrypt_response::ReEncryptResponse;

pub use crate::operation::re_encrypt::_re_encrypt_request::ReEncryptRequest;

pub(crate) mod _re_encrypt_response;

pub(crate) mod _re_encrypt_request;

/// Builders
pub mod builders;
