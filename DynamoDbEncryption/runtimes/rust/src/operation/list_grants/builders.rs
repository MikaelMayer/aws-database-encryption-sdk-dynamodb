// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::list_grants::_list_grants_response::ListGrantsResponseBuilder;

pub use crate::operation::list_grants::_list_grants_request::ListGrantsRequestBuilder;

impl ListGrantsRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::list_grants::ListGrantsResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.list_grants();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGrants`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGrantsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::list_grants::builders::ListGrantsRequestBuilder,
}
impl ListGrantsFluentBuilder {
    /// Creates a new `ListGrants`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ListGrants as a reference.
    pub fn as_input(&self) -> &crate::operation::list_grants::builders::ListGrantsRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_grants::ListGrantsResponse,
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
        crate::operation::list_grants::ListGrants::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn grant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.grant_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_grant_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_grant_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn grantee_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.grantee_principal(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grantee_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_grantee_principal(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grantee_principal(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_grantee_principal()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_id()
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
