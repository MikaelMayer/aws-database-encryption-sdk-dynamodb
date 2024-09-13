// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GenerateEccKeyPair`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GenerateEccKeyPair;
impl GenerateEccKeyPair {
    /// Creates a new `GenerateEccKeyPair`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::generate_ecc_key_pair::GenerateEccKeyPairInput,
    ) -> ::std::result::Result<
        crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::generate_ecc_key_pair::_generate_ecc_key_pair_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GenerateECCKeyPair(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::generate_ecc_key_pair::_generate_ecc_key_pair_output::from_dafny(
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

pub use crate::operation::generate_ecc_key_pair::_generate_ecc_key_pair_output::GenerateEccKeyPairOutput;

pub use crate::operation::generate_ecc_key_pair::_generate_ecc_key_pair_input::GenerateEccKeyPairInput;

pub(crate) mod _generate_ecc_key_pair_output;

pub(crate) mod _generate_ecc_key_pair_input;

/// Builders
pub mod builders;