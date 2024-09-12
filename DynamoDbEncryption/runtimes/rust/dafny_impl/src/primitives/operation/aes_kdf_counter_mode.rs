// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `AesKdfCounterMode`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AesKdfCounterMode;
impl AesKdfCounterMode {
    /// Creates a new `AesKdfCounterMode`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::aes_kdf_counter_mode::AesKdfCtrInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::aes_kdf_counter_mode::_aes_kdf_counter_mode_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).AesKdfCounterMode(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::aes_kdf_counter_mode::_aes_kdf_counter_mode_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::aes_kdf_counter_mode::_aes_kdf_ctr_output::AesKdfCtrOutput;

pub use crate::primitives::operation::aes_kdf_counter_mode::_aes_kdf_ctr_input::AesKdfCtrInput;

pub(crate) mod _aes_kdf_ctr_output;

pub(crate) mod _aes_kdf_ctr_input;

/// Builders
pub mod builders;
