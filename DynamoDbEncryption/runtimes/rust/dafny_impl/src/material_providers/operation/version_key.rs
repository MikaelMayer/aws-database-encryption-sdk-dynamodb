// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `VersionKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VersionKey;
impl VersionKey {
    /// Creates a new `VersionKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::material_providers::client::Client,
        input: crate::material_providers::operation::version_key::VersionKeyInput,
    ) -> ::std::result::Result<
        crate::material_providers::operation::version_key::VersionKeyOutput,
        crate::material_providers::types::error::Error,
    > {
                let inner_input = crate::material_providers::conversions::version_key::_version_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).VersionKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::material_providers::conversions::version_key::_version_key_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::material_providers::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::material_providers::operation::version_key::_version_key_output::VersionKeyOutput;

pub use crate::material_providers::operation::version_key::_version_key_input::VersionKeyInput;

pub(crate) mod _version_key_output;

pub(crate) mod _version_key_input;

/// Builders
pub mod builders;
