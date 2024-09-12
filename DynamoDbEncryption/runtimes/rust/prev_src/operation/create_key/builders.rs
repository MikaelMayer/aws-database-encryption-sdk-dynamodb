// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_key::_create_key_response::CreateKeyResponseBuilder;

pub use crate::operation::create_key::_create_key_request::CreateKeyRequestBuilder;

impl CreateKeyRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateKeyFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_key::builders::CreateKeyRequestBuilder,
}
impl CreateKeyFluentBuilder {
    /// Creates a new `CreateKey`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateKey as a reference.
    pub fn as_input(&self) -> &crate::operation::create_key::builders::CreateKeyRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_key::CreateKeyResponse,
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
        crate::operation::create_key::CreateKey::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn bypass_policy_lockout_safety_check(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.inner = self.inner.bypass_policy_lockout_safety_check(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_bypass_policy_lockout_safety_check(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.inner = self.inner.set_bypass_policy_lockout_safety_check(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_bypass_policy_lockout_safety_check(&self) -> &::std::option::Option<::std::primitive::bool> {
    self.inner.get_bypass_policy_lockout_safety_check()
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
pub fn customer_master_key_spec(mut self, input: impl ::std::convert::Into<kms::types::CustomerMasterKeySpec>) -> Self {
    self.inner = self.inner.customer_master_key_spec(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_customer_master_key_spec(mut self, input: ::std::option::Option<kms::types::CustomerMasterKeySpec>) -> Self {
    self.inner = self.inner.set_customer_master_key_spec(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_customer_master_key_spec(&self) -> &::std::option::Option<kms::types::CustomerMasterKeySpec> {
    self.inner.get_customer_master_key_spec()
}
#[allow(missing_docs)] // documentation missing in model
pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.description(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_description(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_description()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_spec(mut self, input: impl ::std::convert::Into<kms::types::KeySpec>) -> Self {
    self.inner = self.inner.key_spec(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_spec(mut self, input: ::std::option::Option<kms::types::KeySpec>) -> Self {
    self.inner = self.inner.set_key_spec(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_spec(&self) -> &::std::option::Option<kms::types::KeySpec> {
    self.inner.get_key_spec()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_usage(mut self, input: impl ::std::convert::Into<kms::types::KeyUsageType>) -> Self {
    self.inner = self.inner.key_usage(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_usage(mut self, input: ::std::option::Option<kms::types::KeyUsageType>) -> Self {
    self.inner = self.inner.set_key_usage(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_usage(&self) -> &::std::option::Option<kms::types::KeyUsageType> {
    self.inner.get_key_usage()
}
#[allow(missing_docs)] // documentation missing in model
pub fn multi_region(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.inner = self.inner.multi_region(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_multi_region(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.inner = self.inner.set_multi_region(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_multi_region(&self) -> &::std::option::Option<::std::primitive::bool> {
    self.inner.get_multi_region()
}
#[allow(missing_docs)] // documentation missing in model
pub fn origin(mut self, input: impl ::std::convert::Into<kms::types::OriginType>) -> Self {
    self.inner = self.inner.origin(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_origin(mut self, input: ::std::option::Option<kms::types::OriginType>) -> Self {
    self.inner = self.inner.set_origin(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_origin(&self) -> &::std::option::Option<kms::types::OriginType> {
    self.inner.get_origin()
}
#[allow(missing_docs)] // documentation missing in model
pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.policy(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_policy(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_policy()
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.inner = self.inner.tags(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.inner = self.inner.set_tags(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    self.inner.get_tags()
}
#[allow(missing_docs)] // documentation missing in model
pub fn xks_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.xks_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_xks_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_xks_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_xks_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_xks_key_id()
}
}
