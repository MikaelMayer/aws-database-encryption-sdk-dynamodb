// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_details: ::std::option::Option<dynamodb::types::BackupDetails>,
}
impl CreateBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_details(&self) -> &::std::option::Option<dynamodb::types::BackupDetails> {
    &self.backup_details
}
}
impl CreateBackupOutput {
    /// Creates a new builder-style object to manufacture [`CreateBackupOutput`](crate::operation::create_backup::builders::CreateBackupOutput).
    pub fn builder() -> crate::operation::create_backup::builders::CreateBackupOutputBuilder {
        crate::operation::create_backup::builders::CreateBackupOutputBuilder::default()
    }
}

/// A builder for [`CreateBackupOutput`](crate::operation::operation::CreateBackupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateBackupOutputBuilder {
    pub(crate) backup_details: ::std::option::Option<dynamodb::types::BackupDetails>,
}
impl CreateBackupOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_details(mut self, input: impl ::std::convert::Into<dynamodb::types::BackupDetails>) -> Self {
    self.backup_details = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_details(mut self, input: ::std::option::Option<dynamodb::types::BackupDetails>) -> Self {
    self.backup_details = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_details(&self) -> &::std::option::Option<dynamodb::types::BackupDetails> {
    &self.backup_details
}
    /// Consumes the builder and constructs a [`CreateBackupOutput`](crate::operation::operation::CreateBackupOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_backup::CreateBackupOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_backup::CreateBackupOutput {
            backup_details: self.backup_details,
        })
    }
}
