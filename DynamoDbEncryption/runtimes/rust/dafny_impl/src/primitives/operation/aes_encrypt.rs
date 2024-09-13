// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `AesEncrypt`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AesEncrypt;
impl AesEncrypt {
    /// Creates a new `AesEncrypt`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::aes_encrypt::AesEncryptInput,
    ) -> ::std::result::Result<
        crate::primitives::operation::aes_encrypt::AesEncryptOutput,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::aes_encrypt::_aes_encrypt_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).AESEncrypt(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::aes_encrypt::_aes_encrypt_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::aes_encrypt::_aes_encrypt_output::AesEncryptOutput;

pub use crate::primitives::operation::aes_encrypt::_aes_encrypt_input::AesEncryptInput;

pub(crate) mod _aes_encrypt_output;

pub(crate) mod _aes_encrypt_input;

/// Builders
pub mod builders;