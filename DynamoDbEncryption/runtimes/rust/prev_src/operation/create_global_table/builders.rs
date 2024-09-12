// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_global_table::_create_global_table_output::CreateGlobalTableOutputBuilder;

pub use crate::operation::create_global_table::_create_global_table_input::CreateGlobalTableInputBuilder;

impl CreateGlobalTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_global_table::CreateGlobalTableOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_global_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateGlobalTable`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateGlobalTableFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_global_table::builders::CreateGlobalTableInputBuilder,
}
impl CreateGlobalTableFluentBuilder {
    /// Creates a new `CreateGlobalTable`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateGlobalTable as a reference.
    pub fn as_input(&self) -> &crate::operation::create_global_table::builders::CreateGlobalTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_global_table::CreateGlobalTableOutput,
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
        crate::operation::create_global_table::CreateGlobalTable::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.global_table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_global_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_global_table_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn replication_group(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Replica>>) -> Self {
    self.inner = self.inner.replication_group(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replication_group(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>>) -> Self {
    self.inner = self.inner.set_replication_group(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replication_group(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>> {
    self.inner.get_replication_group()
}
}
