// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_arn
}
}
impl DeleteBackupInput {
    /// Creates a new builder-style object to manufacture [`DeleteBackupInput`](crate::operation::delete_backup::builders::DeleteBackupInput).
    pub fn builder() -> crate::operation::delete_backup::builders::DeleteBackupInputBuilder {
        crate::operation::delete_backup::builders::DeleteBackupInputBuilder::default()
    }
}

/// A builder for [`DeleteBackupInput`](crate::operation::operation::DeleteBackupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteBackupInputBuilder {
    pub(crate) backup_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteBackupInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteBackupInput`](crate::operation::operation::DeleteBackupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_backup::DeleteBackupInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_backup::DeleteBackupInput {
            backup_arn: self.backup_arn,
        })
    }
}
