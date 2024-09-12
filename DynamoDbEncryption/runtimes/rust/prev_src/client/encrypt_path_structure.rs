// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`EncryptPathStructure`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<material_providers::types::DbeAlgorithmSuiteId>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<material_providers::types::DbeAlgorithmSuiteId>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`cmm(impl Into<Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::cmm) / [`set_cmm(Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::set_cmm): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`plaintext_structure(impl Into<Option<::std::vec::Vec<structured_encryption::types::CryptoItem>>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::plaintext_structure) / [`set_plaintext_structure(Option<::std::vec::Vec<structured_encryption::types::CryptoItem>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::set_plaintext_structure): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`EncryptPathStructureOutput`](crate::operation::encrypt_path_structure::EncryptPathStructureOutput) with field(s):
    ///   - [`encrypted_structure(Option<::std::vec::Vec<structured_encryption::types::CryptoItem>>)`](crate::operation::encrypt_path_structure::EncryptPathStructureOutput::encrypted_structure): (undocumented)
    ///   - [`parsed_header(Option<structured_encryption::types::ParsedHeader>)`](crate::operation::encrypt_path_structure::EncryptPathStructureOutput::parsed_header): (undocumented)
    /// - On failure, responds with [`SdkError<EncryptPathStructureError>`](crate::operation::encrypt_path_structure::EncryptPathStructureError)
    pub fn encrypt_path_structure(&self) -> crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder {
        crate::operation::encrypt_path_structure::builders::EncryptPathStructureFluentBuilder::new(self.clone())
    }
}
