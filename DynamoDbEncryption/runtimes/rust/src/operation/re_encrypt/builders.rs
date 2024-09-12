// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::re_encrypt::_re_encrypt_response::ReEncryptResponseBuilder;

pub use crate::operation::re_encrypt::_re_encrypt_request::ReEncryptRequestBuilder;

impl ReEncryptRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::re_encrypt::ReEncryptResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.re_encrypt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ReEncrypt`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReEncryptFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::re_encrypt::builders::ReEncryptRequestBuilder,
}
impl ReEncryptFluentBuilder {
    /// Creates a new `ReEncrypt`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ReEncrypt as a reference.
    pub fn as_input(&self) -> &crate::operation::re_encrypt::builders::ReEncryptRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::re_encrypt::ReEncryptResponse,
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
        crate::operation::re_encrypt::ReEncrypt::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.ciphertext_blob(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ciphertext_blob(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_ciphertext_blob(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_ciphertext_blob()
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.destination_encryption_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.set_destination_encryption_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    self.inner.get_destination_encryption_algorithm()
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.destination_encryption_context(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.set_destination_encryption_context(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    self.inner.get_destination_encryption_context()
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.destination_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_destination_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_destination_key_id()
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
pub fn source_encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.source_encryption_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.set_source_encryption_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    self.inner.get_source_encryption_algorithm()
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.source_encryption_context(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.inner = self.inner.set_source_encryption_context(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    self.inner.get_source_encryption_context()
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.source_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_source_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_source_key_id()
}
}
