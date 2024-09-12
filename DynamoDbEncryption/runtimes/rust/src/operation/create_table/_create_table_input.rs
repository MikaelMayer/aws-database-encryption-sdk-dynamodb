// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub attribute_definitions: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>,
#[allow(missing_docs)] // documentation missing in model
pub billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
#[allow(missing_docs)] // documentation missing in model
pub global_secondary_indexes: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub key_schema: ::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>>,
#[allow(missing_docs)] // documentation missing in model
pub local_secondary_indexes: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
#[allow(missing_docs)] // documentation missing in model
pub provisioned_throughput: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
#[allow(missing_docs)] // documentation missing in model
pub sse_specification: ::std::option::Option<dynamodb::types::SseSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub stream_specification: ::std::option::Option<dynamodb::types::StreamSpecification>,
#[allow(missing_docs)] // documentation missing in model
pub table_class: ::std::option::Option<dynamodb::types::TableClass>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl CreateTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_definitions(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>> {
    &self.attribute_definitions
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    &self.billing_mode
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    &self.global_secondary_indexes
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_schema(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>> {
    &self.key_schema
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    &self.local_secondary_indexes
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    &self.provisioned_throughput
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
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    &self.tags
}
}
impl CreateTableInput {
    /// Creates a new builder-style object to manufacture [`CreateTableInput`](crate::operation::create_table::builders::CreateTableInput).
    pub fn builder() -> crate::operation::create_table::builders::CreateTableInputBuilder {
        crate::operation::create_table::builders::CreateTableInputBuilder::default()
    }
}

/// A builder for [`CreateTableInput`](crate::operation::operation::CreateTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTableInputBuilder {
    pub(crate) attribute_definitions: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>,
pub(crate) billing_mode: ::std::option::Option<dynamodb::types::BillingMode>,
pub(crate) global_secondary_indexes: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>,
pub(crate) key_schema: ::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>>,
pub(crate) local_secondary_indexes: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>,
pub(crate) provisioned_throughput: ::std::option::Option<dynamodb::types::ProvisionedThroughput>,
pub(crate) sse_specification: ::std::option::Option<dynamodb::types::SseSpecification>,
pub(crate) stream_specification: ::std::option::Option<dynamodb::types::StreamSpecification>,
pub(crate) table_class: ::std::option::Option<dynamodb::types::TableClass>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl CreateTableInputBuilder {
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
pub fn global_secondary_indexes(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.global_secondary_indexes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.global_secondary_indexes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    &self.global_secondary_indexes
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_schema(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::KeySchemaElement>>) -> Self {
    self.key_schema = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_schema(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>>) -> Self {
    self.key_schema = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_schema(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>> {
    &self.key_schema
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_indexes(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.local_secondary_indexes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_local_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.local_secondary_indexes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_local_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    &self.local_secondary_indexes
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
#[allow(missing_docs)] // documentation missing in model
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.tags = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.tags = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    &self.tags
}
    /// Consumes the builder and constructs a [`CreateTableInput`](crate::operation::operation::CreateTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_table::CreateTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_table::CreateTableInput {
            attribute_definitions: self.attribute_definitions,
billing_mode: self.billing_mode,
global_secondary_indexes: self.global_secondary_indexes,
key_schema: self.key_schema,
local_secondary_indexes: self.local_secondary_indexes,
provisioned_throughput: self.provisioned_throughput,
sse_specification: self.sse_specification,
stream_specification: self.stream_specification,
table_class: self.table_class,
table_name: self.table_name,
tags: self.tags,
        })
    }
}
