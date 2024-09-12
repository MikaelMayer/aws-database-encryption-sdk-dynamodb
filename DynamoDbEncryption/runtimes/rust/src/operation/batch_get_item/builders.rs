// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::batch_get_item::_batch_get_item_output::BatchGetItemOutputBuilder;

pub use crate::operation::batch_get_item::_batch_get_item_input::BatchGetItemInputBuilder;

impl BatchGetItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_get_item::BatchGetItemOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.batch_get_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchGetItem`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGetItemFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::batch_get_item::builders::BatchGetItemInputBuilder,
}
impl BatchGetItemFluentBuilder {
    /// Creates a new `BatchGetItem`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the BatchGetItem as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_get_item::builders::BatchGetItemInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_get_item::BatchGetItemOutput,
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
        crate::operation::batch_get_item::BatchGetItem::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn request_items(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.inner = self.inner.request_items(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_request_items(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.inner = self.inner.set_request_items(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>> {
    self.inner.get_request_items()
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
}
