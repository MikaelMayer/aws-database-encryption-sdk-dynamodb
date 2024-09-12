// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub attribute_definitions: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>,
#[allow(missing_docs)] // documentation missing in model
pub billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
#[allow(missing_docs)] // documentation missing in model
pub global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>,
#[allow(missing_docs)] // documentation missing in model
pub provisioned_throughput: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
#[allow(missing_docs)] // documentation missing in model
pub replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>,
#[allow(missing_docs)] // documentation missing in model
pub sse_specification: ::std::option::Option<dynamodb::types::SseSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub stream_specification: ::std::option::Option<dynamodb::types::StreamSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub table_class: ::std::option::Option<dynamodb::types::TableClass>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_definitions(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>> {
    &self.attribute_definitions
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.billing_mode
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>> {
    &self.global_secondary_index_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    &self.provisioned_throughput
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>> {
    &self.replica_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    &self.sse_specification
}
#[allow(missing_docs)] // documentation missing in model
pub fn stream_specification(&self) -> &::std::option::Option<dynamodb::types::StreamSpecification> {
    &self.stream_specification
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_class(&self) -> &::std::option::Option<dynamodb::types::TableClass> {
    &self.table_class
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl UpdateTableInput {
    /// Creates a new builder-style object to manufacture [`UpdateTableInput`](crate::operation::update_table::builders::UpdateTableInput).
    pub fn builder() -> crate::operation::update_table::builders::UpdateTableInputBuilder {
        crate::operation::update_table::builders::UpdateTableInputBuilder::default()
    }
}

/// A builder for [`UpdateTableInput`](crate::operation::operation::UpdateTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateTableInputBuilder {
    pub(crate) attribute_definitions: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>,
pub(crate) billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
pub(crate) global_secondary_index_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>,
pub(crate) provisioned_throughput: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
pub(crate) replica_updates: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>,
pub(crate) sse_specification: ::std::option::Option<dynamodb::types::SseSpecification>,
pub(crate) stream_specification: ::std::option::Option<dynamodb::types::StreamSpecification>,
pub(crate) table_class: ::std::option::Option<dynamodb::types::TableClass>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_definitions(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::AttributeDefinition>>) -> Self {
    self.attribute_definitions = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>) -> Self {
    self.attribute_definitions = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attribute_definitions(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>> {
    &self.attribute_definitions
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.billing_mode = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_billing_mode(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.billing_mode = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.billing_mode
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_index_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>) -> Self {
    self.global_secondary_index_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>) -> Self {
    self.global_secondary_index_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>> {
    &self.global_secondary_index_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput(mut self, input: impl ::std::convert::Into<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.provisioned_throughput = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_throughput(mut self, input: ::std::option::Option<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.provisioned_throughput = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_throughput(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    &self.provisioned_throughput
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>) -> Self {
    self.replica_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>) -> Self {
    self.replica_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>> {
    &self.replica_updates
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::SseSpecification>) -> Self {
    self.sse_specification = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_sse_specification(mut self, input: ::std::option::Option<dynamodb::types::SseSpecification>) -> Self {
    self.sse_specification = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_sse_specification(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    &self.sse_specification
}
#[allow(missing_docs)] // documentation missing in model
pub fn stream_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::StreamSpecification>) -> Self {
    self.stream_specification = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_stream_specification(mut self, input: ::std::option::Option<dynamodb::types::StreamSpecification>) -> Self {
    self.stream_specification = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_stream_specification(&self) -> &::std::option::Option<dynamodb::types::StreamSpecification> {
    &self.stream_specification
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_class(mut self, input: impl ::std::convert::Into<dynamodb::types::TableClass>) -> Self {
    self.table_class = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_class(mut self, input: ::std::option::Option<dynamodb::types::TableClass>) -> Self {
    self.table_class = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_class(&self) -> &::std::option::Option<dynamodb::types::TableClass> {
    &self.table_class
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
    /// Consumes the builder and constructs a [`UpdateTableInput`](crate::operation::operation::UpdateTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_table::UpdateTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_table::UpdateTableInput {
            attribute_definitions: self.attribute_definitions,
billing_mode: self.billing_mode,
global_secondary_index_updates: self.global_secondary_index_updates,
provisioned_throughput: self.provisioned_throughput,
replica_updates: self.replica_updates,
sse_specification: self.sse_specification,
stream_specification: self.stream_specification,
table_class: self.table_class,
table_name: self.table_name,
        })
    }
}
