// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EcdsaSign`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EcdsaSign;
impl EcdsaSign {
    /// Creates a new `EcdsaSign`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::ecdsa_sign::EcdsaSignInput,
    ) -> ::std::result::Result<
        ::aws_smithy_types::Blob,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::ecdsa_sign::_ecdsa_sign_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ECDSASign(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::ecdsa_sign::_ecdsa_sign_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::ecdsa_sign::_ecdsa_sign_output::EcdsaSignOutput;

pub use crate::primitives::operation::ecdsa_sign::_ecdsa_sign_input::EcdsaSignInput;

pub(crate) mod _ecdsa_sign_output;

pub(crate) mod _ecdsa_sign_input;

/// Builders
pub mod builders;
