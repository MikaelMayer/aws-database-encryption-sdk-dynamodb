// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_aws_kms_rsa_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::operation::create_aws_kms_rsa_keyring::_create_aws_kms_rsa_keyring_input::CreateAwsKmsRsaKeyringInputBuilder;

impl CreateAwsKmsRsaKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        material_providers::types::keyring::KeyringRef,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_rsa_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsRsaKeyring`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsRsaKeyringFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringInputBuilder,
}
impl CreateAwsKmsRsaKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsRsaKeyring`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsRsaKeyring as a reference.
    pub fn as_input(&self) -> &crate::operation::create_aws_kms_rsa_keyring::builders::CreateAwsKmsRsaKeyringInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        material_providers::types::keyring::KeyringRef,
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
        crate::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyring::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.encryption_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.inner = self.inner.set_encryption_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    self.inner.get_encryption_algorithm()
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
pub fn kms_client(mut self, input: impl ::std::convert::Into<kms::client::Client>) -> Self {
    self.inner = self.inner.kms_client(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_kms_client(mut self, input: ::std::option::Option<kms::client::Client>) -> Self {
    self.inner = self.inner.set_kms_client(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_kms_client(&self) -> &::std::option::Option<kms::client::Client> {
    self.inner.get_kms_client()
}
#[allow(missing_docs)] // documentation missing in model
pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.kms_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_kms_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_kms_key_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.public_key(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_public_key(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_public_key()
}
}
