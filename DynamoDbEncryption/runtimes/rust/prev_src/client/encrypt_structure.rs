// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`EncryptStructure`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite_id(impl Into<Option<material_providers::types::DbeAlgorithmSuiteId>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::algorithm_suite_id) / [`set_algorithm_suite_id(Option<material_providers::types::DbeAlgorithmSuiteId>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_algorithm_suite_id): (undocumented)<br>
    ///   - [`cmm(impl Into<Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::cmm) / [`set_cmm(Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_cmm): (undocumented)<br>
    ///   - [`crypto_schema(impl Into<Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::crypto_schema) / [`set_crypto_schema(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_crypto_schema): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`plaintext_structure(impl Into<Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::plaintext_structure) / [`set_plaintext_structure(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_plaintext_structure): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`EncryptStructureOutput`](crate::operation::encrypt_structure::EncryptStructureOutput) with field(s):
    ///   - [`crypto_schema(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>)`](crate::operation::encrypt_structure::EncryptStructureOutput::crypto_schema): (undocumented)
    ///   - [`encrypted_structure(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>)`](crate::operation::encrypt_structure::EncryptStructureOutput::encrypted_structure): (undocumented)
    ///   - [`parsed_header(Option<structured_encryption::types::ParsedHeader>)`](crate::operation::encrypt_structure::EncryptStructureOutput::parsed_header): (undocumented)
    /// - On failure, responds with [`SdkError<EncryptStructureError>`](crate::operation::encrypt_structure::EncryptStructureError)
    pub fn encrypt_structure(&self) -> crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder {
        crate::operation::encrypt_structure::builders::EncryptStructureFluentBuilder::new(self.clone())
    }
}
