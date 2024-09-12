// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateRequiredEncryptionContextCMM`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyring(impl Into<Option<material_providers::types::keyring::KeyringRef>>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::keyring) / [`set_keyring(Option<material_providers::types::keyring::KeyringRef>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::set_keyring): (undocumented)<br>
    ///   - [`required_encryption_context_keys(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::required_encryption_context_keys) / [`set_required_encryption_context_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::set_required_encryption_context_keys): (undocumented)<br>
    ///   - [`underlying_cmm(impl Into<Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::underlying_cmm) / [`set_underlying_cmm(Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCMMFluentBuilder::set_underlying_cmm): (undocumented)<br>
    /// - On success, responds with [`CreateRequiredEncryptionContextCmmOutput`](crate::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmOutput) with field(s):
    ///   - [`materials_manager(Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCMMOutput::materials_manager): (undocumented)
    /// - On failure, responds with [`SdkError<CreateRequiredEncryptionContextCmmError>`](crate::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmError)
    pub fn create_required_encryption_context_cmm(&self) -> crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmFluentBuilder {
        crate::operation::create_required_encryption_context_cmm::builders::CreateRequiredEncryptionContextCmmFluentBuilder::new(self.clone())
    }
}
