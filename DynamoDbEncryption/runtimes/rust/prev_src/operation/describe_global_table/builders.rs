// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::describe_global_table::_describe_global_table_output::DescribeGlobalTableOutputBuilder;

pub use crate::operation::describe_global_table::_describe_global_table_input::DescribeGlobalTableInputBuilder;

impl DescribeGlobalTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table::DescribeGlobalTableOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.describe_global_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeGlobalTable`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeGlobalTableFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::describe_global_table::builders::DescribeGlobalTableInputBuilder,
}
impl DescribeGlobalTableFluentBuilder {
    /// Creates a new `DescribeGlobalTable`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeGlobalTable as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_global_table::builders::DescribeGlobalTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table::DescribeGlobalTableOutput,
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
        crate::operation::describe_global_table::DescribeGlobalTable::send(&self.client, input).await
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
}
