// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateContinuousBackups`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateContinuousBackups;
impl UpdateContinuousBackups {
    /// Creates a new `UpdateContinuousBackups`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_continuous_backups::UpdateContinuousBackupsInput,
    ) -> ::std::result::Result<
        crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_continuous_backups::_update_continuous_backups_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateContinuousBackups(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_continuous_backups::_update_continuous_backups_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::update_continuous_backups::_update_continuous_backups_output::UpdateContinuousBackupsOutput;

pub use crate::operation::update_continuous_backups::_update_continuous_backups_input::UpdateContinuousBackupsInput;

pub(crate) mod _update_continuous_backups_output;

pub(crate) mod _update_continuous_backups_input;

/// Builders
pub mod builders;
