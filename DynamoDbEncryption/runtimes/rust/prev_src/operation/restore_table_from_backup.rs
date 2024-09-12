// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `RestoreTableFromBackup`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RestoreTableFromBackup;
impl RestoreTableFromBackup {
    /// Creates a new `RestoreTableFromBackup`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::restore_table_from_backup::RestoreTableFromBackupInput,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::restore_table_from_backup::_restore_table_from_backup_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).RestoreTableFromBackup(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::restore_table_from_backup::_restore_table_from_backup_output::from_dafny(
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

pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_output::RestoreTableFromBackupOutput;

pub use crate::operation::restore_table_from_backup::_restore_table_from_backup_input::RestoreTableFromBackupInput;

pub(crate) mod _restore_table_from_backup_output;

pub(crate) mod _restore_table_from_backup_input;

/// Builders
pub mod builders;
