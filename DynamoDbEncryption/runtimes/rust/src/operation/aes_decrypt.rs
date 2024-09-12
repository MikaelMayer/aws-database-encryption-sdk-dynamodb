// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `AesDecrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AesDecrypt;
impl AesDecrypt {
    /// Creates a new `AesDecrypt`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::aes_decrypt::AesDecryptInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::aes_decrypt::_aes_decrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).AESDecrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::aes_decrypt::_aes_decrypt_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::aes_decrypt::_aes_decrypt_output::AesDecryptOutput;

pub use crate::operation::aes_decrypt::_aes_decrypt_input::AesDecryptInput;

pub(crate) mod _aes_decrypt_output;

pub(crate) mod _aes_decrypt_input;

/// Builders
pub mod builders;
