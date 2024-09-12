// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteStatementInput {
    #[allow(missing_docs)] // documentation missing in model
pub consistent_read: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub parameters: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub statement: ::std::option::Option<::std::string::String>,
}
impl ExecuteStatementInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consistent_read(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.consistent_read
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn parameters(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>> {
    &self.parameters
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn statement(&self) -> &::std::option::Option<::std::string::String> {
    &self.statement
}
}
impl ExecuteStatementInput {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInput`](crate::operation::execute_statement::builders::ExecuteStatementInput).
    pub fn builder() -> crate::operation::execute_statement::builders::ExecuteStatementInputBuilder {
        crate::operation::execute_statement::builders::ExecuteStatementInputBuilder::default()
    }
}

/// A builder for [`ExecuteStatementInput`](crate::operation::operation::ExecuteStatementInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecuteStatementInputBuilder {
    pub(crate) consistent_read: ::std::option::Option<::std::primitive::bool>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) parameters: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) statement: ::std::option::Option<::std::string::String>,
}
impl ExecuteStatementInputBuilder {
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
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.next_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.next_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn parameters(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::AttributeValue>>) -> Self {
    self.parameters = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>>) -> Self {
    self.parameters = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>> {
    &self.parameters
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
pub fn statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.statement = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.statement = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_statement(&self) -> &::std::option::Option<::std::string::String> {
    &self.statement
}
    /// Consumes the builder and constructs a [`ExecuteStatementInput`](crate::operation::operation::ExecuteStatementInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::execute_statement::ExecuteStatementInput {
            consistent_read: self.consistent_read,
limit: self.limit,
next_token: self.next_token,
parameters: self.parameters,
return_consumed_capacity: self.return_consumed_capacity,
statement: self.statement,
        })
    }
}
