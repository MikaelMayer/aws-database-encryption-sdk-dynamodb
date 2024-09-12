// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::list_global_tables::_list_global_tables_output::ListGlobalTablesOutputBuilder;

pub use crate::operation::list_global_tables::_list_global_tables_input::ListGlobalTablesInputBuilder;

impl ListGlobalTablesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.list_global_tables();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGlobalTables`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGlobalTablesFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder,
}
impl ListGlobalTablesFluentBuilder {
    /// Creates a new `ListGlobalTables`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ListGlobalTables as a reference.
    pub fn as_input(&self) -> &crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
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
        crate::operation::list_global_tables::ListGlobalTables::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.exclusive_start_global_table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_exclusive_start_global_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_exclusive_start_global_table_name()
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
pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.region_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_region_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_region_name()
}
}
