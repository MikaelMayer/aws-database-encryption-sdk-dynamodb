// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub attributes_to_get: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub consistent_read: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub projection_expression: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl GetItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attributes_to_get(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.attributes_to_get
}
#[allow(missing_docs)] // documentation missing in model
pub fn consistent_read(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.consistent_read
}
#[allow(missing_docs)] // documentation missing in model
pub fn expression_attribute_names(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.expression_attribute_names
}
#[allow(missing_docs)] // documentation missing in model
pub fn key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.key
}
#[allow(missing_docs)] // documentation missing in model
pub fn projection_expression(&self) -> &::std::option::Option<::std::string::String> {
    &self.projection_expression
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl GetItemInput {
    /// Creates a new builder-style object to manufacture [`GetItemInput`](crate::operation::get_item::builders::GetItemInput).
    pub fn builder() -> crate::operation::get_item::builders::GetItemInputBuilder {
        crate::operation::get_item::builders::GetItemInputBuilder::default()
    }
}

/// A builder for [`GetItemInput`](crate::operation::operation::GetItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetItemInputBuilder {
    pub(crate) attributes_to_get: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) consistent_read: ::std::option::Option<::std::primitive::bool>,
pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) projection_expression: ::std::option::Option<::std::string::String>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl GetItemInputBuilder {
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
    /// Consumes the builder and constructs a [`GetItemInput`](crate::operation::operation::GetItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_item::GetItemInput {
            attributes_to_get: self.attributes_to_get,
consistent_read: self.consistent_read,
expression_attribute_names: self.expression_attribute_names,
key: self.key,
projection_expression: self.projection_expression,
return_consumed_capacity: self.return_consumed_capacity,
table_name: self.table_name,
        })
    }
}
