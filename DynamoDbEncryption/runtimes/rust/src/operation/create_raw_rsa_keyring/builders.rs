// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_raw_rsa_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::operation::create_raw_rsa_keyring::_create_raw_rsa_keyring_input::CreateRawRsaKeyringInputBuilder;

impl CreateRawRsaKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        material_providers::types::keyring::KeyringRef,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_raw_rsa_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRawRsaKeyring`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRawRsaKeyringFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder,
}
impl CreateRawRsaKeyringFluentBuilder {
    /// Creates a new `CreateRawRsaKeyring`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateRawRsaKeyring as a reference.
    pub fn as_input(&self) -> &crate::operation::create_raw_rsa_keyring::builders::CreateRawRsaKeyringInputBuilder {
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
        crate::operation::create_raw_rsa_keyring::CreateRawRsaKeyring::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_namespace(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_namespace(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_namespace(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_namespace()
}
#[allow(missing_docs)] // documentation missing in model
pub fn padding_scheme(mut self, input: impl ::std::convert::Into<material_providers::types::PaddingScheme>) -> Self {
    self.inner = self.inner.padding_scheme(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_padding_scheme(mut self, input: ::std::option::Option<material_providers::types::PaddingScheme>) -> Self {
    self.inner = self.inner.set_padding_scheme(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_padding_scheme(&self) -> &::std::option::Option<material_providers::types::PaddingScheme> {
    self.inner.get_padding_scheme()
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.private_key(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_private_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_private_key(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_private_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_private_key()
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
