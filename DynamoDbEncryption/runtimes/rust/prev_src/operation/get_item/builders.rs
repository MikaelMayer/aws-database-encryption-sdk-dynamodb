// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::get_item::_get_item_output::GetItemOutputBuilder;

pub use crate::operation::get_item::_get_item_input::GetItemInputBuilder;

impl GetItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.get_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetItem`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetItemFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::get_item::builders::GetItemInputBuilder,
}
impl GetItemFluentBuilder {
    /// Creates a new `GetItem`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetItem as a reference.
    pub fn as_input(&self) -> &crate::operation::get_item::builders::GetItemInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemOutput,
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
        crate::operation::get_item::GetItem::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn attributes_to_get(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.attributes_to_get(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attributes_to_get(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.set_attributes_to_get(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attributes_to_get(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    self.inner.get_attributes_to_get()
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
pub fn expression_attribute_names(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.expression_attribute_names(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expression_attribute_names(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.set_expression_attribute_names(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expression_attribute_names(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    self.inner.get_expression_attribute_names()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.key(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.set_key(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    self.inner.get_key()
}
#[allow(missing_docs)] // documentation missing in model
pub fn projection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.projection_expression(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_projection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_projection_expression(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_projection_expression(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_projection_expression()
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
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
}
