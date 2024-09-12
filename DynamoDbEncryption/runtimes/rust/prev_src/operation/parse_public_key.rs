// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ParsePublicKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ParsePublicKey;
impl ParsePublicKey {
    /// Creates a new `ParsePublicKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::parse_public_key::ParsePublicKeyInput,
    ) -> ::std::result::Result<
        crate::operation::parse_public_key::ParsePublicKeyOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::parse_public_key::_parse_public_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ParsePublicKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::parse_public_key::_parse_public_key_output::from_dafny(
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

pub use crate::operation::parse_public_key::_parse_public_key_output::ParsePublicKeyOutput;

pub use crate::operation::parse_public_key::_parse_public_key_input::ParsePublicKeyInput;

pub(crate) mod _parse_public_key_output;

pub(crate) mod _parse_public_key_input;

/// Builders
pub mod builders;
