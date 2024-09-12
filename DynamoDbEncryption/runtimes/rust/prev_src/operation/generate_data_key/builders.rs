// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::generate_data_key::_generate_data_key_response::GenerateDataKeyResponseBuilder;

pub use crate::operation::generate_data_key::_generate_data_key_request::GenerateDataKeyRequestBuilder;

impl GenerateDataKeyRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key::GenerateDataKeyResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.generate_data_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateDataKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateDataKeyFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::generate_data_key::builders::GenerateDataKeyRequestBuilder,
}
impl GenerateDataKeyFluentBuilder {
    /// Creates a new `GenerateDataKey`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateDataKey as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_data_key::builders::GenerateDataKeyRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key::GenerateDataKeyResponse,
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
        crate::operation::generate_data_key::GenerateDataKey::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.inner = self.inner.dry_run(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_dry_run(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.inner = self.inner.set_dry_run(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    self.inner.get_dry_run()
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.encryption_context(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.set_encryption_context(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    self.inner.get_encryption_context()
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.grant_tokens(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.set_grant_tokens(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    self.inner.get_grant_tokens()
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
pub fn key_spec(mut self, input: impl ::std::convert::Into<kms::types::DataKeySpec>) -> Self {
    self.inner = self.inner.key_spec(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_spec(mut self, input: ::std::option::Option<kms::types::DataKeySpec>) -> Self {
    self.inner = self.inner.set_key_spec(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_spec(&self) -> &::std::option::Option<kms::types::DataKeySpec> {
    self.inner.get_key_spec()
}
#[allow(missing_docs)] // documentation missing in model
pub fn number_of_bytes(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.number_of_bytes(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_number_of_bytes(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_number_of_bytes(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_number_of_bytes(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_number_of_bytes()
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(mut self, input: impl ::std::convert::Into<kms::types::RecipientInfo>) -> Self {
    self.inner = self.inner.recipient(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_recipient(mut self, input: ::std::option::Option<kms::types::RecipientInfo>) -> Self {
    self.inner = self.inner.set_recipient(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    self.inner.get_recipient()
}
}
