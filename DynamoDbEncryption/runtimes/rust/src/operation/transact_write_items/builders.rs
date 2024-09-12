// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::transact_write_items::_transact_write_items_output::TransactWriteItemsOutputBuilder;

pub use crate::operation::transact_write_items::_transact_write_items_input::TransactWriteItemsInputBuilder;

impl TransactWriteItemsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::transact_write_items::TransactWriteItemsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.transact_write_items();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TransactWriteItems`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TransactWriteItemsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::transact_write_items::builders::TransactWriteItemsInputBuilder,
}
impl TransactWriteItemsFluentBuilder {
    /// Creates a new `TransactWriteItems`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the TransactWriteItems as a reference.
    pub fn as_input(&self) -> &crate::operation::transact_write_items::builders::TransactWriteItemsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_write_items::TransactWriteItemsOutput,
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
        crate::operation::transact_write_items::TransactWriteItems::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.client_request_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_client_request_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_client_request_token()
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
pub fn return_item_collection_metrics(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.inner = self.inner.return_item_collection_metrics(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.inner = self.inner.set_return_item_collection_metrics(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    self.inner.get_return_item_collection_metrics()
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::TransactWriteItem>>) -> Self {
    self.inner = self.inner.transact_items(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transact_items(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>) -> Self {
    self.inner = self.inner.set_transact_items(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>> {
    self.inner.get_transact_items()
}
}
