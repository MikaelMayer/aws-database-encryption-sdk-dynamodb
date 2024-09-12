// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ValidatePublicKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ValidatePublicKey;
impl ValidatePublicKey {
    /// Creates a new `ValidatePublicKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::validate_public_key::ValidatePublicKeyInput,
    ) -> ::std::result::Result<
        crate::primitives::operation::validate_public_key::ValidatePublicKeyOutput,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::validate_public_key::_validate_public_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ValidatePublicKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::validate_public_key::_validate_public_key_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::validate_public_key::_validate_public_key_output::ValidatePublicKeyOutput;

pub use crate::primitives::operation::validate_public_key::_validate_public_key_input::ValidatePublicKeyInput;

pub(crate) mod _validate_public_key_output;

pub(crate) mod _validate_public_key_input;

/// Builders
pub mod builders;
