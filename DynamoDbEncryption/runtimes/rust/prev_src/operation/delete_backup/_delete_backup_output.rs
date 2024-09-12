// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_description: ::std::option::Option<dynamodb::types::BackupDescription>,
}
impl DeleteBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_description(&self) -> &::std::option::Option<dynamodb::types::BackupDescription> {
    &self.backup_description
}
}
impl DeleteBackupOutput {
    /// Creates a new builder-style object to manufacture [`DeleteBackupOutput`](crate::operation::delete_backup::builders::DeleteBackupOutput).
    pub fn builder() -> crate::operation::delete_backup::builders::DeleteBackupOutputBuilder {
        crate::operation::delete_backup::builders::DeleteBackupOutputBuilder::default()
    }
}

/// A builder for [`DeleteBackupOutput`](crate::operation::operation::DeleteBackupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteBackupOutputBuilder {
    pub(crate) backup_description: ::std::option::Option<dynamodb::types::BackupDescription>,
}
impl DeleteBackupOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_description(mut self, input: impl ::std::convert::Into<dynamodb::types::BackupDescription>) -> Self {
    self.backup_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_description(mut self, input: ::std::option::Option<dynamodb::types::BackupDescription>) -> Self {
    self.backup_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_description(&self) -> &::std::option::Option<dynamodb::types::BackupDescription> {
    &self.backup_description
}
    /// Consumes the builder and constructs a [`DeleteBackupOutput`](crate::operation::operation::DeleteBackupOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_backup::DeleteBackupOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_backup::DeleteBackupOutput {
            backup_description: self.backup_description,
        })
    }
}
