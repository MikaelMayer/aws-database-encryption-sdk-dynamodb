// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchExecuteStatementInput {
    #[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub statements: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>,
}
impl BatchExecuteStatementInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn statements(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>> {
    &self.statements
}
}
impl BatchExecuteStatementInput {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementInput`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementInput).
    pub fn builder() -> crate::operation::batch_execute_statement::builders::BatchExecuteStatementInputBuilder {
        crate::operation::batch_execute_statement::builders::BatchExecuteStatementInputBuilder::default()
    }
}

/// A builder for [`BatchExecuteStatementInput`](crate::operation::operation::BatchExecuteStatementInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchExecuteStatementInputBuilder {
    pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) statements: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>,
}
impl BatchExecuteStatementInputBuilder {
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
pub fn statements(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>) -> Self {
    self.statements = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_statements(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>) -> Self {
    self.statements = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_statements(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>> {
    &self.statements
}
    /// Consumes the builder and constructs a [`BatchExecuteStatementInput`](crate::operation::operation::BatchExecuteStatementInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_execute_statement::BatchExecuteStatementInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_execute_statement::BatchExecuteStatementInput {
            return_consumed_capacity: self.return_consumed_capacity,
statements: self.statements,
        })
    }
}
