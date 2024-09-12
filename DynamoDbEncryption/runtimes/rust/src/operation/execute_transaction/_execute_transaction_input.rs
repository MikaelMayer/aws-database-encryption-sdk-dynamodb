// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecuteTransactionInput {
    #[allow(missing_docs)] // documentation missing in model
pub client_request_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub transact_statements: ::std::option::Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>,
}
impl ExecuteTransactionInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_request_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_request_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_statements(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>> {
    &self.transact_statements
}
}
impl ExecuteTransactionInput {
    /// Creates a new builder-style object to manufacture [`ExecuteTransactionInput`](crate::operation::execute_transaction::builders::ExecuteTransactionInput).
    pub fn builder() -> crate::operation::execute_transaction::builders::ExecuteTransactionInputBuilder {
        crate::operation::execute_transaction::builders::ExecuteTransactionInputBuilder::default()
    }
}

/// A builder for [`ExecuteTransactionInput`](crate::operation::operation::ExecuteTransactionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecuteTransactionInputBuilder {
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) transact_statements: ::std::option::Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>,
}
impl ExecuteTransactionInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.client_request_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.client_request_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_request_token
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
pub fn transact_statements(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>) -> Self {
    self.transact_statements = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transact_statements(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>) -> Self {
    self.transact_statements = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transact_statements(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>> {
    &self.transact_statements
}
    /// Consumes the builder and constructs a [`ExecuteTransactionInput`](crate::operation::operation::ExecuteTransactionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_transaction::ExecuteTransactionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::execute_transaction::ExecuteTransactionInput {
            client_request_token: self.client_request_token,
return_consumed_capacity: self.return_consumed_capacity,
transact_statements: self.transact_statements,
        })
    }
}
