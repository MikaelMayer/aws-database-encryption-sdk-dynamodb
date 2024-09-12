// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub attribute_updates: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>>,
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
pub key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
#[allow(missing_docs)] // documentation missing in model
pub return_values: ::std::option::Option<dynamodb::types::ReturnValue>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub update_expression: ::std::option::Option<::std::string::String>,
}
impl UpdateItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_updates(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>> {
    &self.attribute_updates
}
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
pub fn key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.key
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
#[allow(missing_docs)] // documentation missing in model
pub fn update_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.update_expression
}
}
impl UpdateItemInput {
    /// Creates a new builder-style object to manufacture [`UpdateItemInput`](crate::operation::update_item::builders::UpdateItemInput).
    pub fn builder() -> crate::operation::update_item::builders::UpdateItemInputBuilder {
        crate::operation::update_item::builders::UpdateItemInputBuilder::default()
    }
}

/// A builder for [`UpdateItemInput`](crate::operation::operation::UpdateItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateItemInputBuilder {
    pub(crate) attribute_updates: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>>,
pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
pub(crate) conditional_operator: ::std::option::Option<dynamodb::types::ConditionalOperator>,
pub(crate) expected: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>,
pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
pub(crate) return_values: ::std::option::Option<dynamodb::types::ReturnValue>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
pub(crate) update_expression: ::std::option::Option<::std::string::String>,
}
impl UpdateItemInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_updates(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>>) -> Self {
    self.attribute_updates = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attribute_updates(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>>) -> Self {
    self.attribute_updates = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attribute_updates(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValueUpdate>> {
    &self.attribute_updates
}
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
pub fn key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.key
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
#[allow(missing_docs)] // documentation missing in model
pub fn update_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.update_expression = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_update_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.update_expression = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_update_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.update_expression
}
    /// Consumes the builder and constructs a [`UpdateItemInput`](crate::operation::operation::UpdateItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_item::UpdateItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_item::UpdateItemInput {
            attribute_updates: self.attribute_updates,
condition_expression: self.condition_expression,
conditional_operator: self.conditional_operator,
expected: self.expected,
expression_attribute_names: self.expression_attribute_names,
expression_attribute_values: self.expression_attribute_values,
key: self.key,
return_consumed_capacity: self.return_consumed_capacity,
return_item_collection_metrics: self.return_item_collection_metrics,
return_values: self.return_values,
table_name: self.table_name,
update_expression: self.update_expression,
        })
    }
}
