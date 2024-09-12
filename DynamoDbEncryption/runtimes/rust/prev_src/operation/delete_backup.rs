// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DeleteBackup`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteBackup;
impl DeleteBackup {
    /// Creates a new `DeleteBackup`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::delete_backup::DeleteBackupInput,
    ) -> ::std::result::Result<
        crate::operation::delete_backup::DeleteBackupOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::delete_backup::_delete_backup_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DeleteBackup(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::delete_backup::_delete_backup_output::from_dafny(
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

pub use crate::operation::delete_backup::_delete_backup_output::DeleteBackupOutput;

pub use crate::operation::delete_backup::_delete_backup_input::DeleteBackupInput;

pub(crate) mod _delete_backup_output;

pub(crate) mod _delete_backup_input;

/// Builders
pub mod builders;
