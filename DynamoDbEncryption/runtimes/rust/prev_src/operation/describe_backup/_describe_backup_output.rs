// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_description: ::std::option::Option<dynamodb::types::BackupDescription>,
}
impl DescribeBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_description(&self) -> &::std::option::Option<dynamodb::types::BackupDescription> {
    &self.backup_description
}
}
impl DescribeBackupOutput {
    /// Creates a new builder-style object to manufacture [`DescribeBackupOutput`](crate::operation::describe_backup::builders::DescribeBackupOutput).
    pub fn builder() -> crate::operation::describe_backup::builders::DescribeBackupOutputBuilder {
        crate::operation::describe_backup::builders::DescribeBackupOutputBuilder::default()
    }
}

/// A builder for [`DescribeBackupOutput`](crate::operation::operation::DescribeBackupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeBackupOutputBuilder {
    pub(crate) backup_description: ::std::option::Option<dynamodb::types::BackupDescription>,
}
impl DescribeBackupOutputBuilder {
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
    /// Consumes the builder and constructs a [`DescribeBackupOutput`](crate::operation::operation::DescribeBackupOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_backup::DescribeBackupOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_backup::DescribeBackupOutput {
            backup_description: self.backup_description,
        })
    }
}
