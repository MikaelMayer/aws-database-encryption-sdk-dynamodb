// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::transact_get_items::_transact_get_items_output::TransactGetItemsOutputBuilder;

pub use crate::operation::transact_get_items::_transact_get_items_input::TransactGetItemsInputBuilder;

impl TransactGetItemsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.transact_get_items();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TransactGetItems`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TransactGetItemsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder,
}
impl TransactGetItemsFluentBuilder {
    /// Creates a new `TransactGetItems`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the TransactGetItems as a reference.
    pub fn as_input(&self) -> &crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsOutput,
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
        crate::operation::transact_get_items::TransactGetItems::send(&self.client, input).await
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
pub fn transact_items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::TransactGetItem>>) -> Self {
    self.inner = self.inner.transact_items(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transact_items(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>) -> Self {
    self.inner = self.inner.set_transact_items(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>> {
    self.inner.get_transact_items()
}
}
