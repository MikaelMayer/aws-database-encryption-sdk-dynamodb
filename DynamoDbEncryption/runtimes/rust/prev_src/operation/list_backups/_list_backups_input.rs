// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_type: ::std::option::Option<dynamodb::types::BackupTypeFilter>,
#[allow(missing_docs)] // documentation missing in model
pub exclusive_start_backup_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub time_range_lower_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub time_range_upper_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListBackupsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_type(&self) -> &::std::option::Option<dynamodb::types::BackupTypeFilter> {
    &self.backup_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.exclusive_start_backup_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_lower_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.time_range_lower_bound
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_upper_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.time_range_upper_bound
}
}
impl ListBackupsInput {
    /// Creates a new builder-style object to manufacture [`ListBackupsInput`](crate::operation::list_backups::builders::ListBackupsInput).
    pub fn builder() -> crate::operation::list_backups::builders::ListBackupsInputBuilder {
        crate::operation::list_backups::builders::ListBackupsInputBuilder::default()
    }
}

/// A builder for [`ListBackupsInput`](crate::operation::operation::ListBackupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListBackupsInputBuilder {
    pub(crate) backup_type: ::std::option::Option<dynamodb::types::BackupTypeFilter>,
pub(crate) exclusive_start_backup_arn: ::std::option::Option<::std::string::String>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
pub(crate) time_range_lower_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) time_range_upper_bound: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListBackupsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_type(mut self, input: impl ::std::convert::Into<dynamodb::types::BackupTypeFilter>) -> Self {
    self.backup_type = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_type(mut self, input: ::std::option::Option<dynamodb::types::BackupTypeFilter>) -> Self {
    self.backup_type = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_type(&self) -> &::std::option::Option<dynamodb::types::BackupTypeFilter> {
    &self.backup_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.exclusive_start_backup_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.exclusive_start_backup_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.exclusive_start_backup_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.limit = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_limit(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.limit = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_lower_bound(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.time_range_lower_bound = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_range_lower_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.time_range_lower_bound = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_range_lower_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.time_range_lower_bound
}
#[allow(missing_docs)] // documentation missing in model
pub fn time_range_upper_bound(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.time_range_upper_bound = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_time_range_upper_bound(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.time_range_upper_bound = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_time_range_upper_bound(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.time_range_upper_bound
}
    /// Consumes the builder and constructs a [`ListBackupsInput`](crate::operation::operation::ListBackupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_backups::ListBackupsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_backups::ListBackupsInput {
            backup_type: self.backup_type,
exclusive_start_backup_arn: self.exclusive_start_backup_arn,
limit: self.limit,
table_name: self.table_name,
time_range_lower_bound: self.time_range_lower_bound,
time_range_upper_bound: self.time_range_upper_bound,
        })
    }
}
