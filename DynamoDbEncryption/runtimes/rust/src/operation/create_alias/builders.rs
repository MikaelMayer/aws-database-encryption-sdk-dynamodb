// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_alias::_unit::UnitBuilder;

pub use crate::operation::create_alias::_create_alias_request::CreateAliasRequestBuilder;

impl CreateAliasRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_alias::Unit,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_alias();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAlias`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAliasFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_alias::builders::CreateAliasRequestBuilder,
}
impl CreateAliasFluentBuilder {
    /// Creates a new `CreateAlias`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAlias as a reference.
    pub fn as_input(&self) -> &crate::operation::create_alias::builders::CreateAliasRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_alias::Unit,
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
        crate::operation::create_alias::CreateAlias::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn alias_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.alias_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_alias_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_alias_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_alias_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_alias_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.target_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_target_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_target_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_target_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_target_key_id()
}
}
