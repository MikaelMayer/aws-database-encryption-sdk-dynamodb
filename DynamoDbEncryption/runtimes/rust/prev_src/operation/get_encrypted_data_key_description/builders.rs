// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::get_encrypted_data_key_description::_get_encrypted_data_key_description_output::GetEncryptedDataKeyDescriptionOutputBuilder;

pub use crate::operation::get_encrypted_data_key_description::_get_encrypted_data_key_description_input::GetEncryptedDataKeyDescriptionInputBuilder;

impl GetEncryptedDataKeyDescriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescriptionOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.get_encrypted_data_key_description();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEncryptedDataKeyDescription`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEncryptedDataKeyDescriptionFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::get_encrypted_data_key_description::builders::GetEncryptedDataKeyDescriptionInputBuilder,
}
impl GetEncryptedDataKeyDescriptionFluentBuilder {
    /// Creates a new `GetEncryptedDataKeyDescription`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetEncryptedDataKeyDescription as a reference.
    pub fn as_input(&self) -> &crate::operation::get_encrypted_data_key_description::builders::GetEncryptedDataKeyDescriptionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescriptionOutput,
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
        crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescription::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn input(mut self, input: impl ::std::convert::Into<dynamo_db::types::GetEncryptedDataKeyDescriptionUnion>) -> Self {
    self.inner = self.inner.input(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input(mut self, input: ::std::option::Option<dynamo_db::types::GetEncryptedDataKeyDescriptionUnion>) -> Self {
    self.inner = self.inner.set_input(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input(&self) -> &::std::option::Option<dynamo_db::types::GetEncryptedDataKeyDescriptionUnion> {
    self.inner.get_input()
}
}
