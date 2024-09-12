// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `RsaDecrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RsaDecrypt;
impl RsaDecrypt {
    /// Creates a new `RsaDecrypt`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::rsa_decrypt::RsaDecryptInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::rsa_decrypt::_rsa_decrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).RSADecrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::rsa_decrypt::_rsa_decrypt_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::rsa_decrypt::_rsa_decrypt_output::RsaDecryptOutput;

pub use crate::operation::rsa_decrypt::_rsa_decrypt_input::RsaDecryptInput;

pub(crate) mod _rsa_decrypt_output;

pub(crate) mod _rsa_decrypt_input;

/// Builders
pub mod builders;
