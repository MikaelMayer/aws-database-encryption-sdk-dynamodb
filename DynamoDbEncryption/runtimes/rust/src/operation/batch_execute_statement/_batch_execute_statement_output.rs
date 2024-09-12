// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchExecuteStatementOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
#[allow(missing_docs)] // documentation missing in model
pub responses: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>>,
}
impl BatchExecuteStatementOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn responses(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>> {
    &self.responses
}
}
impl BatchExecuteStatementOutput {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementOutput`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementOutput).
    pub fn builder() -> crate::operation::batch_execute_statement::builders::BatchExecuteStatementOutputBuilder {
        crate::operation::batch_execute_statement::builders::BatchExecuteStatementOutputBuilder::default()
    }
}

/// A builder for [`BatchExecuteStatementOutput`](crate::operation::operation::BatchExecuteStatementOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchExecuteStatementOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
pub(crate) responses: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>>,
}
impl BatchExecuteStatementOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consumed_capacity(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn responses(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::BatchStatementResponse>>) -> Self {
    self.responses = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_responses(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>>) -> Self {
    self.responses = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_responses(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>> {
    &self.responses
}
    /// Consumes the builder and constructs a [`BatchExecuteStatementOutput`](crate::operation::operation::BatchExecuteStatementOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_execute_statement::BatchExecuteStatementOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_execute_statement::BatchExecuteStatementOutput {
            consumed_capacity: self.consumed_capacity,
responses: self.responses,
        })
    }
}
