// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::describe_custom_key_stores::_describe_custom_key_stores_response::DescribeCustomKeyStoresResponseBuilder;

pub use crate::operation::describe_custom_key_stores::_describe_custom_key_stores_request::DescribeCustomKeyStoresRequestBuilder;

impl DescribeCustomKeyStoresRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.describe_custom_key_stores();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeCustomKeyStores`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeCustomKeyStoresFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresRequestBuilder,
}
impl DescribeCustomKeyStoresFluentBuilder {
    /// Creates a new `DescribeCustomKeyStores`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeCustomKeyStores as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse,
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
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStores::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.custom_key_store_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_custom_key_store_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_custom_key_store_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.custom_key_store_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_custom_key_store_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_custom_key_store_name()
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
pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.marker(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_marker(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_marker()
}
}
