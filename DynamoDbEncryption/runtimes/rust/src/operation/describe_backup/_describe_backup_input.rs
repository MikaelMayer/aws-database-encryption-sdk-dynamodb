// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_arn
}
}
impl DescribeBackupInput {
    /// Creates a new builder-style object to manufacture [`DescribeBackupInput`](crate::operation::describe_backup::builders::DescribeBackupInput).
    pub fn builder() -> crate::operation::describe_backup::builders::DescribeBackupInputBuilder {
        crate::operation::describe_backup::builders::DescribeBackupInputBuilder::default()
    }
}

/// A builder for [`DescribeBackupInput`](crate::operation::operation::DescribeBackupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeBackupInputBuilder {
    pub(crate) backup_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeBackupInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.backup_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.backup_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_arn
}
    /// Consumes the builder and constructs a [`DescribeBackupInput`](crate::operation::operation::DescribeBackupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_backup::DescribeBackupInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_backup::DescribeBackupInput {
            backup_arn: self.backup_arn,
        })
    }
}
