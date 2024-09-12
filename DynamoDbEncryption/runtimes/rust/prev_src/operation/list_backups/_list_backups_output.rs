// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListBackupsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::BackupSummary>>,
#[allow(missing_docs)] // documentation missing in model
pub last_evaluated_backup_arn: ::std::option::Option<::std::string::String>,
}
impl ListBackupsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BackupSummary>> {
    &self.backup_summaries
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_backup_arn
}
}
impl ListBackupsOutput {
    /// Creates a new builder-style object to manufacture [`ListBackupsOutput`](crate::operation::list_backups::builders::ListBackupsOutput).
    pub fn builder() -> crate::operation::list_backups::builders::ListBackupsOutputBuilder {
        crate::operation::list_backups::builders::ListBackupsOutputBuilder::default()
    }
}

/// A builder for [`ListBackupsOutput`](crate::operation::operation::ListBackupsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListBackupsOutputBuilder {
    pub(crate) backup_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::BackupSummary>>,
pub(crate) last_evaluated_backup_arn: ::std::option::Option<::std::string::String>,
}
impl ListBackupsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_summaries(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::BackupSummary>>) -> Self {
    self.backup_summaries = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_summaries(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::BackupSummary>>) -> Self {
    self.backup_summaries = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BackupSummary>> {
    &self.backup_summaries
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.last_evaluated_backup_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_evaluated_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.last_evaluated_backup_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_evaluated_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_backup_arn
}
    /// Consumes the builder and constructs a [`ListBackupsOutput`](crate::operation::operation::ListBackupsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_backups::ListBackupsOutput {
            backup_summaries: self.backup_summaries,
last_evaluated_backup_arn: self.last_evaluated_backup_arn,
        })
    }
}
