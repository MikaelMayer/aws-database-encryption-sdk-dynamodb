// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetActiveBranchKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetActiveBranchKey;
impl GetActiveBranchKey {
    /// Creates a new `GetActiveBranchKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::material_providers::client::Client,
        input: crate::material_providers::operation::get_active_branch_key::GetActiveBranchKeyInput,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_active_branch_key::GetActiveBranchKeyOutput,
        crate::material_providers::types::error::Error,
    > {
                let inner_input = crate::material_providers::conversions::get_active_branch_key::_get_active_branch_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetActiveBranchKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::material_providers::conversions::get_active_branch_key::_get_active_branch_key_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::material_providers::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::material_providers::operation::get_active_branch_key::_get_active_branch_key_output::GetActiveBranchKeyOutput;

pub use crate::material_providers::operation::get_active_branch_key::_get_active_branch_key_input::GetActiveBranchKeyInput;

pub(crate) mod _get_active_branch_key_output;

pub(crate) mod _get_active_branch_key_input;

/// Builders
pub mod builders;