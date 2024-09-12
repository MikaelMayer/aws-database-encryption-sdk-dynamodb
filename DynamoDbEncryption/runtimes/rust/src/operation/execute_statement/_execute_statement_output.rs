// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteStatementOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>,
#[allow(missing_docs)] // documentation missing in model
pub last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
}
impl ExecuteStatementOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn items(&self) -> &::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>> {
    &self.items
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.last_evaluated_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
}
impl ExecuteStatementOutput {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementOutput`](crate::operation::execute_statement::builders::ExecuteStatementOutput).
    pub fn builder() -> crate::operation::execute_statement::builders::ExecuteStatementOutputBuilder {
        crate::operation::execute_statement::builders::ExecuteStatementOutputBuilder::default()
    }
}

/// A builder for [`ExecuteStatementOutput`](crate::operation::operation::ExecuteStatementOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecuteStatementOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>,
pub(crate) last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ExecuteStatementOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ConsumedCapacity>) -> Self {
    self.consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ConsumedCapacity>) -> Self {
    self.consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>) -> Self {
    self.items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_items(mut self, input: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>) -> Self {
    self.items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_items(&self) -> &::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>> {
    &self.items
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.last_evaluated_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_evaluated_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.last_evaluated_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.last_evaluated_key
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
    /// Consumes the builder and constructs a [`ExecuteStatementOutput`](crate::operation::operation::ExecuteStatementOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::execute_statement::ExecuteStatementOutput {
            consumed_capacity: self.consumed_capacity,
items: self.items,
last_evaluated_key: self.last_evaluated_key,
next_token: self.next_token,
        })
    }
}
