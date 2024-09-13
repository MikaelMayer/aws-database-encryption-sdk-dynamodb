// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetPublicKeyFromPrivateKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetPublicKeyFromPrivateKey;
impl GetPublicKeyFromPrivateKey {
    /// Creates a new `GetPublicKeyFromPrivateKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyInput,
    ) -> ::std::result::Result<
        crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::get_public_key_from_private_key::_get_public_key_from_private_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetPublicKeyFromPrivateKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::get_public_key_from_private_key::_get_public_key_from_private_key_output::from_dafny(
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

pub use crate::operation::get_public_key_from_private_key::_get_public_key_from_private_key_output::GetPublicKeyFromPrivateKeyOutput;

pub use crate::operation::get_public_key_from_private_key::_get_public_key_from_private_key_input::GetPublicKeyFromPrivateKeyInput;

pub(crate) mod _get_public_key_from_private_key_output;

pub(crate) mod _get_public_key_from_private_key_input;

/// Builders
pub mod builders;