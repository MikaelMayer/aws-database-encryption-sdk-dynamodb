// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EcdsaVerify`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EcdsaVerify;
impl EcdsaVerify {
    /// Creates a new `EcdsaVerify`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::ecdsa_verify::EcdsaVerifyInput,
    ) -> ::std::result::Result<
        ::std::primitive::bool,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::ecdsa_verify::_ecdsa_verify_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ECDSAVerify(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::ecdsa_verify::_ecdsa_verify_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::ecdsa_verify::_ecdsa_verify_output::EcdsaVerifyOutput;

pub use crate::primitives::operation::ecdsa_verify::_ecdsa_verify_input::EcdsaVerifyInput;

pub(crate) mod _ecdsa_verify_output;

pub(crate) mod _ecdsa_verify_input;

/// Builders
pub mod builders;
