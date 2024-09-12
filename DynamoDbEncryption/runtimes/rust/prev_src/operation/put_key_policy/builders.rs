// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::put_key_policy::_unit::UnitBuilder;

pub use crate::operation::put_key_policy::_put_key_policy_request::PutKeyPolicyRequestBuilder;

impl PutKeyPolicyRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::put_key_policy::Unit,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.put_key_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutKeyPolicy`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutKeyPolicyFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::put_key_policy::builders::PutKeyPolicyRequestBuilder,
}
impl PutKeyPolicyFluentBuilder {
    /// Creates a new `PutKeyPolicy`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the PutKeyPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::put_key_policy::builders::PutKeyPolicyRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_key_policy::Unit,
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
        crate::operation::put_key_policy::PutKeyPolicy::send(&self.client, input).await
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
pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.policy_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_policy_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_policy_name()
}
}
