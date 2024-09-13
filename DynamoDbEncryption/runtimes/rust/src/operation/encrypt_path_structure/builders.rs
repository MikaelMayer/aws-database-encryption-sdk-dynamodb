// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::encrypt_path_structure::_encrypt_path_structure_output::EncryptPathStructureOutputBuilder;

pub use crate::operation::encrypt_path_structure::_encrypt_path_structure_input::EncryptPathStructureInputBuilder;

impl EncryptPathStructureInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::encrypt_path_structure::EncryptPathStructureOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.encrypt_path_structure();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EncryptPathStructure`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EncryptPathStructureFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::encrypt_path_structure::builders::EncryptPathStructureInputBuilder,
}
impl EncryptPathStructureFluentBuilder {
    /// Creates a new `EncryptPathStructure`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the EncryptPathStructure as a reference.
    pub fn as_input(&self) -> &crate::operation::encrypt_path_structure::builders::EncryptPathStructureInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt_path_structure::EncryptPathStructureOutput,
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
        crate::operation::encrypt_path_structure::EncryptPathStructure::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn algorithm_suite_id(mut self, input: impl ::std::convert::Into<material_providers::types::DbeAlgorithmSuiteId>) -> Self {
    self.inner = self.inner.algorithm_suite_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_algorithm_suite_id(mut self, input: ::std::option::Option<material_providers::types::DbeAlgorithmSuiteId>) -> Self {
    self.inner = self.inner.set_algorithm_suite_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_algorithm_suite_id(&self) -> &::std::option::Option<material_providers::types::DbeAlgorithmSuiteId> {
    self.inner.get_algorithm_suite_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn cmm(mut self, input: impl ::std::convert::Into<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.cmm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cmm(mut self, input: ::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.inner = self.inner.set_cmm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cmm(&self) -> &::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    self.inner.get_cmm()
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
pub fn plaintext_structure(mut self, input: impl ::std::convert::Into<::std::vec::Vec<structured_encryption::types::CryptoItem>>) -> Self {
    self.inner = self.inner.plaintext_structure(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_plaintext_structure(mut self, input: ::std::option::Option<::std::vec::Vec<structured_encryption::types::CryptoItem>>) -> Self {
    self.inner = self.inner.set_plaintext_structure(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_plaintext_structure(&self) -> &::std::option::Option<::std::vec::Vec<structured_encryption::types::CryptoItem>> {
    self.inner.get_plaintext_structure()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
}