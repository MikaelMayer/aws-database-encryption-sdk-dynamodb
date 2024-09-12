// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_table::_create_table_output::CreateTableOutputBuilder;

pub use crate::operation::create_table::_create_table_input::CreateTableInputBuilder;

impl CreateTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_table::CreateTableOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateTable`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTableFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_table::builders::CreateTableInputBuilder,
}
impl CreateTableFluentBuilder {
    /// Creates a new `CreateTable`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateTable as a reference.
    pub fn as_input(&self) -> &crate::operation::create_table::builders::CreateTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_table::CreateTableOutput,
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
        crate::operation::create_table::CreateTable::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn attribute_definitions(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::AttributeDefinition>>) -> Self {
    self.inner = self.inner.attribute_definitions(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attribute_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>) -> Self {
    self.inner = self.inner.set_attribute_definitions(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attribute_definitions(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>> {
    self.inner.get_attribute_definitions()
}
#[allow(missing_docs)] // documentation missing in model
pub fn billing_mode(mut self, input: impl ::std::convert::Into<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.billing_mode(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_billing_mode(mut self, input: ::std::option::Option<dynamodb::types::BillingMode>) -> Self {
    self.inner = self.inner.set_billing_mode(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_billing_mode(&self) -> &::std::option::Option<dynamodb::types::BillingMode> {
    self.inner.get_billing_mode()
}
#[allow(missing_docs)] // documentation missing in model
pub fn global_secondary_indexes(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.inner = self.inner.global_secondary_indexes(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>>) -> Self {
    self.inner = self.inner.set_global_secondary_indexes(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndex>> {
    self.inner.get_global_secondary_indexes()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_schema(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::KeySchemaElement>>) -> Self {
    self.inner = self.inner.key_schema(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_schema(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>>) -> Self {
    self.inner = self.inner.set_key_schema(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_schema(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::KeySchemaElement>> {
    self.inner.get_key_schema()
}
#[allow(missing_docs)] // documentation missing in model
pub fn local_secondary_indexes(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.inner = self.inner.local_secondary_indexes(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_local_secondary_indexes(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>>) -> Self {
    self.inner = self.inner.set_local_secondary_indexes(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_local_secondary_indexes(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::LocalSecondaryIndex>> {
    self.inner.get_local_secondary_indexes()
}
#[allow(missing_docs)] // documentation missing in model
pub fn provisioned_throughput(mut self, input: impl ::std::convert::Into<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.inner = self.inner.provisioned_throughput(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_provisioned_throughput(mut self, input: ::std::option::Option<dynamodb::types::ProvisionedThroughput>) -> Self {
    self.inner = self.inner.set_provisioned_throughput(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_provisioned_throughput(&self) -> &::std::option::Option<dynamodb::types::ProvisionedThroughput> {
    self.inner.get_provisioned_throughput()
}
#[allow(missing_docs)] // documentation missing in model
pub fn sse_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::SseSpecification>) -> Self {
    self.inner = self.inner.sse_specification(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_sse_specification(mut self, input: ::std::option::Option<dynamodb::types::SseSpecification>) -> Self {
    self.inner = self.inner.set_sse_specification(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_sse_specification(&self) -> &::std::option::Option<dynamodb::types::SseSpecification> {
    self.inner.get_sse_specification()
}
#[allow(missing_docs)] // documentation missing in model
pub fn stream_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::StreamSpecification>) -> Self {
    self.inner = self.inner.stream_specification(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_stream_specification(mut self, input: ::std::option::Option<dynamodb::types::StreamSpecification>) -> Self {
    self.inner = self.inner.set_stream_specification(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_stream_specification(&self) -> &::std::option::Option<dynamodb::types::StreamSpecification> {
    self.inner.get_stream_specification()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_class(mut self, input: impl ::std::convert::Into<dynamodb::types::TableClass>) -> Self {
    self.inner = self.inner.table_class(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_class(mut self, input: ::std::option::Option<dynamodb::types::TableClass>) -> Self {
    self.inner = self.inner.set_table_class(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_class(&self) -> &::std::option::Option<dynamodb::types::TableClass> {
    self.inner.get_table_class()
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
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.inner = self.inner.tags(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.inner = self.inner.set_tags(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    self.inner.get_tags()
}
}
