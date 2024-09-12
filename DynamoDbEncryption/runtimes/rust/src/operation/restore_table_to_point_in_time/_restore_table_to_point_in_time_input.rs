// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreTableToPointInTimeInput {
    #[allow(missing_docs)] // documentation missing in model
pub billing_mode_override: ::std::option::Option<dynamodb::types::BillingMode>,
#[allow(missing_docs)] // documentation missing in model
pub global_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub local_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub provisioned_throughput_override: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
#[allow(missing_docs)] // documentation missing in model
pub restore_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub sse_specification_override: ::std::option::Option<dynamodb::types::SseSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub source_table_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub source_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub target_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub use_latest_restorable_time: ::std::option::Option<::std::primitive::bool>,
}
impl RestoreTableToPointInTimeInput {
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
pub fn restore_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.restore_date_time
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification_override(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    &self.sse_specification_override
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_table_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.target_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn use_latest_restorable_time(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.use_latest_restorable_time
}
}
impl RestoreTableToPointInTimeInput {
    /// Creates a new builder-style object to manufacture [`RestoreTableToPointInTimeInput`](crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeInput).
    pub fn builder() -> crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeInputBuilder {
        crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeInputBuilder::default()
    }
}

/// A builder for [`RestoreTableToPointInTimeInput`](crate::operation::operation::RestoreTableToPointInTimeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreTableToPointInTimeInputBuilder {
    pub(crate) billing_mode_override: ::std::option::Option<dynamodb::types::BillingMode>,
pub(crate) global_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
pub(crate) local_secondary_index_override: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
pub(crate) provisioned_throughput_override: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
pub(crate) restore_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) sse_specification_override: ::std::option::Option<dynamodb::types::SseSpecification>,
pub(crate) source_table_arn: ::std::option::Option<::std::string::String>,
pub(crate) source_table_name: ::std::option::Option<::std::string::String>,
pub(crate) target_table_name: ::std::option::Option<::std::string::String>,
pub(crate) use_latest_restorable_time: ::std::option::Option<::std::primitive::bool>,
}
impl RestoreTableToPointInTimeInputBuilder {
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
pub fn restore_date_time(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.restore_date_time = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_restore_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.restore_date_time = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_restore_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.restore_date_time
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
pub fn source_table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.source_table_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.source_table_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_table_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.source_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.source_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_table_name
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
#[allow(missing_docs)] // documentation missing in model
pub fn use_latest_restorable_time(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.use_latest_restorable_time = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_use_latest_restorable_time(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.use_latest_restorable_time = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_use_latest_restorable_time(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.use_latest_restorable_time
}
    /// Consumes the builder and constructs a [`RestoreTableToPointInTimeInput`](crate::operation::operation::RestoreTableToPointInTimeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput {
            billing_mode_override: self.billing_mode_override,
global_secondary_index_override: self.global_secondary_index_override,
local_secondary_index_override: self.local_secondary_index_override,
provisioned_throughput_override: self.provisioned_throughput_override,
restore_date_time: self.restore_date_time,
sse_specification_override: self.sse_specification_override,
source_table_arn: self.source_table_arn,
source_table_name: self.source_table_name,
target_table_name: self.target_table_name,
use_latest_restorable_time: self.use_latest_restorable_time,
        })
    }
}
