// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::aes_kdf_counter_mode::_aes_kdf_ctr_output::AesKdfCtrOutputBuilder;

pub use crate::operation::aes_kdf_counter_mode::_aes_kdf_ctr_input::AesKdfCtrInputBuilder;

impl AesKdfCtrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::aes_kdf_counter_mode::AesKdfCtrOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.aes_kdf_counter_mode();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AesKdfCounterMode`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AesKdfCounterModeFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::aes_kdf_counter_mode::builders::AesKdfCtrInputBuilder,
}
impl AesKdfCounterModeFluentBuilder {
    /// Creates a new `AesKdfCounterMode`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the AesKdfCounterMode as a reference.
    pub fn as_input(&self) -> &crate::operation::aes_kdf_counter_mode::builders::AesKdfCtrInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::aes_kdf_counter_mode::AesKdfCtrOutput,
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
        crate::operation::aes_kdf_counter_mode::AesKdfCounterMode::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn expected_length(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.expected_length(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expected_length(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_expected_length(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expected_length(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_expected_length()
}
#[allow(missing_docs)] // documentation missing in model
pub fn ikm(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.ikm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ikm(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_ikm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_ikm()
}
#[allow(missing_docs)] // documentation missing in model
pub fn nonce(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.nonce(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_nonce(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_nonce(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_nonce(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_nonce()
}
}