// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::enable_key_rotation::_unit::UnitBuilder;

pub use crate::operation::enable_key_rotation::_enable_key_rotation_request::EnableKeyRotationRequestBuilder;

impl EnableKeyRotationRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_key_rotation::Unit,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.enable_key_rotation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableKeyRotation`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableKeyRotationFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::enable_key_rotation::builders::EnableKeyRotationRequestBuilder,
}
impl EnableKeyRotationFluentBuilder {
    /// Creates a new `EnableKeyRotation`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the EnableKeyRotation as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_key_rotation::builders::EnableKeyRotationRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_key_rotation::Unit,
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
        crate::operation::enable_key_rotation::EnableKeyRotation::send(&self.client, input).await
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
pub fn rotation_period_in_days(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.rotation_period_in_days(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_rotation_period_in_days(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_rotation_period_in_days(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_rotation_period_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_rotation_period_in_days()
}
}
