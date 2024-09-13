// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_required_encryption_context_cmm::_create_required_encryption_context_cmm_output::CreateRequiredEncryptionContextCmmOutputBuilder;

pub use crate::operation::create_required_encryption_context_cmm::_create_required_encryption_context_cmm_input::CreateRequiredEncryptionContextCmmInputBuilder;

impl CreateRequiredEncryptionContextCmmInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_required_encryption_context_cmm();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRequiredEncryptionContextCmm`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRequiredEncryptionContextCmmFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder,
}
impl CreateRequiredEncryptionContextCmmFluentBuilder {
    /// Creates a new `CreateRequiredEncryptionContextCmm`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateRequiredEncryptionContextCmm as a reference.
    pub fn as_input(&self) -> &crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
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
        crate::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmm::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn keyring(mut self, input: impl ::std::convert::Into<material_providers::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.keyring(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_keyring(mut self, input: ::std::option::Option<material_providers::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.set_keyring(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_keyring(&self) -> &::std::option::Option<material_providers::types::keyring::KeyringRef> {
    self.inner.get_keyring()
}
#[allow(missing_docs)] // documentation missing in model
pub fn required_encryption_context_keys(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.required_encryption_context_keys(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_required_encryption_context_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.set_required_encryption_context_keys(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_required_encryption_context_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    self.inner.get_required_encryption_context_keys()
}
#[allow(missing_docs)] // documentation missing in model
pub fn underlying_cmm(mut self, input: impl ::std::convert::Into<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.underlying_cmm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_underlying_cmm(mut self, input: ::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.set_underlying_cmm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_underlying_cmm(&self) -> &::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    self.inner.get_underlying_cmm()
}
}