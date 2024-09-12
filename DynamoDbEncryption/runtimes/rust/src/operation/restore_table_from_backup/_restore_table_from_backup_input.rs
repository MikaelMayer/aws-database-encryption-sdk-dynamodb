// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreTableFromBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub billing_mode_override: ::std::option::Option<dynamodb::types::BillingMode>,
#[allow(missing_docs)] // documentation missing in model
pub global_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub local_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub provisioned_throughput_override: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
#[allow(missing_docs)] // documentation missing in model
pub sse_specification_override: ::std::option::Option<dynamodb::types::SseSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub target_table_name: ::std::option::Option<::std::string::String>,
}
impl RestoreTableFromBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode_override(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.billing_mode_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    &self.global_secondary_index_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    &self.local_secondary_index_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput_override(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    &self.provisioned_throughput_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification_override(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    &self.sse_specification_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.target_table_name
}
}
impl RestoreTableFromBackupInput {
    /// Creates a new builder-style object to manufacture [`RestoreTableFromBackupInput`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInput).
    pub fn builder() -> crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder {
        crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupInputBuilder::default()
    }
}

/// A builder for [`RestoreTableFromBackupInput`](crate::operation::operation::RestoreTableFromBackupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreTableFromBackupInputBuilder {
    pub(crate) backup_arn: ::std::option::Option<::std::string::String>,
pub(crate) billing_mode_override: ::std::option::Option<dynamodb::types::BillingMode>,
pub(crate) global_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
pub(crate) local_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
pub(crate) provisioned_throughput_override: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
pub(crate) sse_specification_override: ::std::option::Option<dynamodb::types::SseSpecification>,
pub(crate) target_table_name: ::std::option::Option<::std::string::String>,
}
impl RestoreTableFromBackupInputBuilder {
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
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode_override(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.billing_mode_override = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_billing_mode_override(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.billing_mode_override = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_billing_mode_override(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.billing_mode_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_override(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.global_secondary_index_override = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.global_secondary_index_override = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    &self.global_secondary_index_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_index_override(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.local_secondary_index_override = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_local_secondary_index_override(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.local_secondary_index_override = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_local_secondary_index_override(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    &self.local_secondary_index_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput_override(mut self, input: impl ::std::convert::Into<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.provisioned_throughput_override = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_throughput_override(mut self, input: ::std::option::Option<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.provisioned_throughput_override = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_throughput_override(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    &self.provisioned_throughput_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification_override(mut self, input: impl ::std::convert::Into<dynamodb::types::SseSpecification>) -> Self {
    self.sse_specification_override = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_sse_specification_override(mut self, input: ::std::option::Option<dynamodb::types::SseSpecification>) -> Self {
    self.sse_specification_override = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_sse_specification_override(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    &self.sse_specification_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.target_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_target_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.target_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_target_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.target_table_name
}
    /// Consumes the builder and constructs a [`RestoreTableFromBackupInput`](crate::operation::operation::RestoreTableFromBackupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::restore_table_from_backup::RestoreTableFromBackupInput {
            backup_arn: self.backup_arn,
billing_mode_override: self.billing_mode_override,
global_secondary_index_override: self.global_secondary_index_override,
local_secondary_index_override: self.local_secondary_index_override,
provisioned_throughput_override: self.provisioned_throughput_override,
sse_specification_override: self.sse_specification_override,
target_table_name: self.target_table_name,
        })
    }
}
