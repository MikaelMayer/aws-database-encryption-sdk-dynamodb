// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub condition_expression: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub conditional_operator: ::std::option::Option<dynamodb::types::ConditionalOperator>,
#[allow(missing_docs)] // documentation missing in model
pub expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
#[allow(missing_docs)] // documentation missing in model
pub return_values: ::std::option::Option<dynamodb::types::ReturnValue>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl PutItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn condition_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.condition_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn conditional_operator(&self) -> &::std::option::Option<dynamodb::types::ConditionalOperator> {
    &self.conditional_operator
}
#[allow(missing_docs)] // documentation missing in model
pub fn expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>> {
    &self.expected
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
pub fn item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.item
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    &self.return_item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_values(&self) -> &::std::option::Option<dynamodb::types::ReturnValue> {
    &self.return_values
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl PutItemInput {
    /// Creates a new builder-style object to manufacture [`PutItemInput`](crate::operation::put_item::builders::PutItemInput).
    pub fn builder() -> crate::operation::put_item::builders::PutItemInputBuilder {
        crate::operation::put_item::builders::PutItemInputBuilder::default()
    }
}

/// A builder for [`PutItemInput`](crate::operation::operation::PutItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutItemInputBuilder {
    pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
pub(crate) conditional_operator: ::std::option::Option<dynamodb::types::ConditionalOperator>,
pub(crate) expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>,
pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
pub(crate) return_values: ::std::option::Option<dynamodb::types::ReturnValue>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl PutItemInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn condition_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.condition_expression = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_condition_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.condition_expression = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.condition_expression
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
pub fn expected(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>) -> Self {
    self.expected = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expected(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>) -> Self {
    self.expected = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>> {
    &self.expected
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
pub fn item(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.item = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.item = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.item
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
pub fn return_item_collection_metrics(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.return_item_collection_metrics = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.return_item_collection_metrics = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    &self.return_item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_values(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnValue>) -> Self {
    self.return_values = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_values(mut self, input: ::std::option::Option<dynamodb::types::ReturnValue>) -> Self {
    self.return_values = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_values(&self) -> &::std::option::Option<dynamodb::types::ReturnValue> {
    &self.return_values
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
    /// Consumes the builder and constructs a [`PutItemInput`](crate::operation::operation::PutItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_item::PutItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_item::PutItemInput {
            condition_expression: self.condition_expression,
conditional_operator: self.conditional_operator,
expected: self.expected,
expression_attribute_names: self.expression_attribute_names,
expression_attribute_values: self.expression_attribute_values,
item: self.item,
return_consumed_capacity: self.return_consumed_capacity,
return_item_collection_metrics: self.return_item_collection_metrics,
return_values: self.return_values,
table_name: self.table_name,
        })
    }
}
