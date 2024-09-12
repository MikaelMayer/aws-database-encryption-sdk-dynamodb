// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::scan::_scan_output::ScanOutputBuilder;

pub use crate::operation::scan::_scan_input::ScanInputBuilder;

impl ScanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.scan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Scan`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ScanFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::scan::builders::ScanInputBuilder,
}
impl ScanFluentBuilder {
    /// Creates a new `Scan`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the Scan as a reference.
    pub fn as_input(&self) -> &crate::operation::scan::builders::ScanInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
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
        crate::operation::scan::Scan::send(&self.client, input).await
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
pub fn exclusive_start_key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.exclusive_start_key(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.inner = self.inner.set_exclusive_start_key(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    self.inner.get_exclusive_start_key()
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
pub fn filter_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.filter_expression(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_filter_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_filter_expression(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_filter_expression(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_filter_expression()
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.index_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_index_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_index_name()
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
pub fn scan_filter(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.inner = self.inner.scan_filter(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_scan_filter(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>>) -> Self {
    self.inner = self.inner.set_scan_filter(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_scan_filter(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::Condition>> {
    self.inner.get_scan_filter()
}
#[allow(missing_docs)] // documentation missing in model
pub fn segment(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.segment(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_segment(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_segment(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_segment(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_segment()
}
#[allow(missing_docs)] // documentation missing in model
pub fn select(mut self, input: impl ::std::convert::Into<dynamodb::types::Select>) -> Self {
    self.inner = self.inner.select(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_select(mut self, input: ::std::option::Option<dynamodb::types::Select>) -> Self {
    self.inner = self.inner.set_select(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_select(&self) -> &::std::option::Option<dynamodb::types::Select> {
    self.inner.get_select()
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
#[allow(missing_docs)] // documentation missing in model
pub fn total_segments(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.total_segments(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_total_segments(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_total_segments(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_total_segments(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_total_segments()
}
}
