// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::batch_execute_statement::_batch_execute_statement_output::BatchExecuteStatementOutputBuilder;

pub use crate::operation::batch_execute_statement::_batch_execute_statement_input::BatchExecuteStatementInputBuilder;

impl BatchExecuteStatementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_execute_statement::BatchExecuteStatementOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.batch_execute_statement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchExecuteStatement`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchExecuteStatementFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::batch_execute_statement::builders::BatchExecuteStatementInputBuilder,
}
impl BatchExecuteStatementFluentBuilder {
    /// Creates a new `BatchExecuteStatement`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the BatchExecuteStatement as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_execute_statement::builders::BatchExecuteStatementInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_execute_statement::BatchExecuteStatementOutput,
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
        crate::operation::batch_execute_statement::BatchExecuteStatement::send(&self.client, input).await
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
pub fn statements(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>) -> Self {
    self.inner = self.inner.statements(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_statements(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>) -> Self {
    self.inner = self.inner.set_statements(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_statements(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>> {
    self.inner.get_statements()
}
}
