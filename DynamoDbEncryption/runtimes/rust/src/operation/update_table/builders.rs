// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::update_table::_update_table_output::UpdateTableOutputBuilder;

pub use crate::operation::update_table::_update_table_input::UpdateTableInputBuilder;

impl UpdateTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::update_table::UpdateTableOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.update_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateTable`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTableFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::update_table::builders::UpdateTableInputBuilder,
}
impl UpdateTableFluentBuilder {
    /// Creates a new `UpdateTable`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the UpdateTable as a reference.
    pub fn as_input(&self) -> &crate::operation::update_table::builders::UpdateTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_table::UpdateTableOutput,
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
        crate::operation::update_table::UpdateTable::send(&self.client, input).await
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
pub fn global_secondary_index_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>) -> Self {
    self.inner = self.inner.global_secondary_index_updates(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_secondary_index_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>) -> Self {
    self.inner = self.inner.set_global_secondary_index_updates(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_secondary_index_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>> {
    self.inner.get_global_secondary_index_updates()
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
pub fn replica_updates(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>) -> Self {
    self.inner = self.inner.replica_updates(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_updates(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>) -> Self {
    self.inner = self.inner.set_replica_updates(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_updates(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>> {
    self.inner.get_replica_updates()
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
}
