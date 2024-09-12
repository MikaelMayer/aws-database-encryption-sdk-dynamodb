// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::execute_statement::_execute_statement_output::ExecuteStatementOutputBuilder;

pub use crate::operation::execute_statement::_execute_statement_input::ExecuteStatementInputBuilder;

impl ExecuteStatementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.execute_statement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExecuteStatement`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecuteStatementFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::execute_statement::builders::ExecuteStatementInputBuilder,
}
impl ExecuteStatementFluentBuilder {
    /// Creates a new `ExecuteStatement`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ExecuteStatement as a reference.
    pub fn as_input(&self) -> &crate::operation::execute_statement::builders::ExecuteStatementInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::execute_statement::ExecuteStatement::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn consistent_read(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.inner = self.inner.consistent_read(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consistent_read(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.inner = self.inner.set_consistent_read(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consistent_read(&self) -> &::std::option::Option<::std::primitive::bool> {
    self.inner.get_consistent_read()
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.limit(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_limit(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_limit(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_limit()
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.next_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_next_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_next_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn parameters(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.parameters(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.set_parameters(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeValue>> {
    self.inner.get_parameters()
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.inner = self.inner.return_consumed_capacity(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.inner = self.inner.set_return_consumed_capacity(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    self.inner.get_return_consumed_capacity()
}
#[allow(missing_docs)] // documentation missing in model
pub fn statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.statement(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_statement(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_statement(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_statement()
}
}
