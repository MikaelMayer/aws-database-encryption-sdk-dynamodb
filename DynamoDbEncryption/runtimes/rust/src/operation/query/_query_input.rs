// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct QueryInput {
    #[allow(missing_docs)] // documentation missing in model
pub attributes_to_get: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub conditional_operator: ::std::option::Option<dynamodb::types::ConditionalOperator>,
#[allow(missing_docs)] // documentation missing in model
pub consistent_read: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub exclusive_start_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub filter_expression: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub index_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_condition_expression: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_conditions: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub projection_expression: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub query_filter: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub scan_index_forward: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub select: ::std::option::Option<dynamodb::types::Select>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl QueryInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attributes_to_get(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.attributes_to_get
}
#[allow(missing_docs)] // documentation missing in model
pub fn conditional_operator(&self) -> &::std::option::Option<dynamodb::types::ConditionalOperator> {
    &self.conditional_operator
}
#[allow(missing_docs)] // documentation missing in model
pub fn consistent_read(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.consistent_read
}
#[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.exclusive_start_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn expression_attribute_names(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.expression_attribute_names
}
#[allow(missing_docs)] // documentation missing in model
pub fn expression_attribute_values(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.expression_attribute_values
}
#[allow(missing_docs)] // documentation missing in model
pub fn filter_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.filter_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.index_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_condition_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_conditions(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>> {
    &self.key_conditions
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn projection_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.projection_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn query_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>> {
    &self.query_filter
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn scan_index_forward(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.scan_index_forward
}
#[allow(missing_docs)] // documentation missing in model
pub fn select(&self) -> &::std::option::Option<dynamodb::types::Select> {
    &self.select
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl QueryInput {
    /// Creates a new builder-style object to manufacture [`QueryInput`](crate::operation::query::builders::QueryInput).
    pub fn builder() -> crate::operation::query::builders::QueryInputBuilder {
        crate::operation::query::builders::QueryInputBuilder::default()
    }
}

/// A builder for [`QueryInput`](crate::operation::operation::QueryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct QueryInputBuilder {
    pub(crate) attributes_to_get: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) conditional_operator: ::std::option::Option<dynamodb::types::ConditionalOperator>,
pub(crate) consistent_read: ::std::option::Option<::std::primitive::bool>,
pub(crate) exclusive_start_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) filter_expression: ::std::option::Option<::std::string::String>,
pub(crate) index_name: ::std::option::Option<::std::string::String>,
pub(crate) key_condition_expression: ::std::option::Option<::std::string::String>,
pub(crate) key_conditions: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
pub(crate) query_filter: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) scan_index_forward: ::std::option::Option<::std::primitive::bool>,
pub(crate) select: ::std::option::Option<dynamodb::types::Select>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl QueryInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn attributes_to_get(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.attributes_to_get = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attributes_to_get(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.attributes_to_get = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attributes_to_get(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.attributes_to_get
}
#[allow(missing_docs)] // documentation missing in model
pub fn conditional_operator(mut self, input: impl ::std::convert::Into<dynamodb::types::ConditionalOperator>) -> Self {
    self.conditional_operator = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_conditional_operator(mut self, input: ::std::option::Option<dynamodb::types::ConditionalOperator>) -> Self {
    self.conditional_operator = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_conditional_operator(&self) -> &::std::option::Option<dynamodb::types::ConditionalOperator> {
    &self.conditional_operator
}
#[allow(missing_docs)] // documentation missing in model
pub fn consistent_read(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.consistent_read = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consistent_read(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.consistent_read = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consistent_read(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.consistent_read
}
#[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.exclusive_start_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.exclusive_start_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.exclusive_start_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn expression_attribute_names(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.expression_attribute_names = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expression_attribute_names(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.expression_attribute_names = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expression_attribute_names(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.expression_attribute_names
}
#[allow(missing_docs)] // documentation missing in model
pub fn expression_attribute_values(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.expression_attribute_values = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expression_attribute_values(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.expression_attribute_values = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expression_attribute_values(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.expression_attribute_values
}
#[allow(missing_docs)] // documentation missing in model
pub fn filter_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.filter_expression = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_filter_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.filter_expression = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_filter_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.filter_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.index_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.index_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.index_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_condition_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_condition_expression = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_condition_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_condition_expression = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_condition_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_conditions(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.key_conditions = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_conditions(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.key_conditions = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_conditions(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>> {
    &self.key_conditions
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
pub fn projection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.projection_expression = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_projection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.projection_expression = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_projection_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.projection_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn query_filter(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.query_filter = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_query_filter(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.query_filter = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_query_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>> {
    &self.query_filter
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn scan_index_forward(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.scan_index_forward = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_scan_index_forward(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.scan_index_forward = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_scan_index_forward(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.scan_index_forward
}
#[allow(missing_docs)] // documentation missing in model
pub fn select(mut self, input: impl ::std::convert::Into<dynamodb::types::Select>) -> Self {
    self.select = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_select(mut self, input: ::std::option::Option<dynamodb::types::Select>) -> Self {
    self.select = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_select(&self) -> &::std::option::Option<dynamodb::types::Select> {
    &self.select
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
    /// Consumes the builder and constructs a [`QueryInput`](crate::operation::operation::QueryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::query::QueryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::query::QueryInput {
            attributes_to_get: self.attributes_to_get,
conditional_operator: self.conditional_operator,
consistent_read: self.consistent_read,
exclusive_start_key: self.exclusive_start_key,
expression_attribute_names: self.expression_attribute_names,
expression_attribute_values: self.expression_attribute_values,
filter_expression: self.filter_expression,
index_name: self.index_name,
key_condition_expression: self.key_condition_expression,
key_conditions: self.key_conditions,
limit: self.limit,
projection_expression: self.projection_expression,
query_filter: self.query_filter,
return_consumed_capacity: self.return_consumed_capacity,
scan_index_forward: self.scan_index_forward,
select: self.select,
table_name: self.table_name,
        })
    }
}
