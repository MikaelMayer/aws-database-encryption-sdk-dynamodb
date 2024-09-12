// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DeriveSharedSecret`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeriveSharedSecret;
impl DeriveSharedSecret {
    /// Creates a new `DeriveSharedSecret`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::primitives::client::Client,
        input: crate::primitives::operation::derive_shared_secret::DeriveSharedSecretInput,
    ) -> ::std::result::Result<
        crate::primitives::operation::derive_shared_secret::DeriveSharedSecretOutput,
        crate::primitives::types::error::Error,
    > {
                let inner_input = crate::primitives::conversions::derive_shared_secret::_derive_shared_secret_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DeriveSharedSecret(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::primitives::conversions::derive_shared_secret::_derive_shared_secret_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::primitives::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::primitives::operation::derive_shared_secret::_derive_shared_secret_output::DeriveSharedSecretOutput;

pub use crate::primitives::operation::derive_shared_secret::_derive_shared_secret_input::DeriveSharedSecretInput;

pub(crate) mod _derive_shared_secret_output;

pub(crate) mod _derive_shared_secret_input;

/// Builders
pub mod builders;
