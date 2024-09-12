// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetBranchKeyVersion`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBranchKeyVersion;
impl GetBranchKeyVersion {
    /// Creates a new `GetBranchKeyVersion`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::get_branch_key_version::GetBranchKeyVersionInput,
    ) -> ::std::result::Result<
        crate::operation::get_branch_key_version::GetBranchKeyVersionOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::get_branch_key_version::_get_branch_key_version_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetBranchKeyVersion(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::get_branch_key_version::_get_branch_key_version_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::get_branch_key_version::_get_branch_key_version_output::GetBranchKeyVersionOutput;

pub use crate::operation::get_branch_key_version::_get_branch_key_version_input::GetBranchKeyVersionInput;

pub(crate) mod _get_branch_key_version_output;

pub(crate) mod _get_branch_key_version_input;

/// Builders
pub mod builders;
