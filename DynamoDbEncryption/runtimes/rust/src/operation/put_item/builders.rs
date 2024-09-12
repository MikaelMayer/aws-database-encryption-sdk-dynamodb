// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::put_item::_put_item_output::PutItemOutputBuilder;

pub use crate::operation::put_item::_put_item_input::PutItemInputBuilder;

impl PutItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::put_item::PutItemOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.put_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutItem`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutItemFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::put_item::builders::PutItemInputBuilder,
}
impl PutItemFluentBuilder {
    /// Creates a new `PutItem`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the PutItem as a reference.
    pub fn as_input(&self) -> &crate::operation::put_item::builders::PutItemInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_item::PutItemOutput,
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
        crate::operation::put_item::PutItem::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn condition_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.condition_expression(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_condition_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_condition_expression(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_condition_expression()
}
#[allow(missing_docs)] // documentation missing in model
pub fn conditional_operator(mut self, input: impl ::std::convert::Into<dynamodb::types::ConditionalOperator>) -> Self {
    self.inner = self.inner.conditional_operator(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_conditional_operator(mut self, input: ::std::option::Option<dynamodb::types::ConditionalOperator>) -> Self {
    self.inner = self.inner.set_conditional_operator(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_conditional_operator(&self) -> &::std::option::Option<dynamodb::types::ConditionalOperator> {
    self.inner.get_conditional_operator()
}
#[allow(missing_docs)] // documentation missing in model
pub fn expected(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>) -> Self {
    self.inner = self.inner.expected(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expected(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>) -> Self {
    self.inner = self.inner.set_expected(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>> {
    self.inner.get_expected()
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
pub fn expression_attribute_values(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.expression_attribute_values(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expression_attribute_values(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.set_expression_attribute_values(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expression_attribute_values(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    self.inner.get_expression_attribute_values()
}
#[allow(missing_docs)] // documentation missing in model
pub fn item(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.item(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.set_item(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    self.inner.get_item()
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
pub fn return_values(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnValue>) -> Self {
    self.inner = self.inner.return_values(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_values(mut self, input: ::std::option::Option<dynamodb::types::ReturnValue>) -> Self {
    self.inner = self.inner.set_return_values(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_values(&self) -> &::std::option::Option<dynamodb::types::ReturnValue> {
    self.inner.get_return_values()
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
