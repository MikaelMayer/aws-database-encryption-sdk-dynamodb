// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DecryptStructure`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`authenticate_schema(impl Into<Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::AuthenticateAction>>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::authenticate_schema) / [`set_authenticate_schema(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::AuthenticateAction>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::set_authenticate_schema): (undocumented)<br>
    ///   - [`cmm(impl Into<Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::cmm) / [`set_cmm(Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::set_cmm): (undocumented)<br>
    ///   - [`encrypted_structure(impl Into<Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::encrypted_structure) / [`set_encrypted_structure(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::set_encrypted_structure): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DecryptStructureOutput`](crate::operation::decrypt_structure::DecryptStructureOutput) with field(s):
    ///   - [`crypto_schema(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>)`](crate::operation::decrypt_structure::DecryptStructureOutput::crypto_schema): (undocumented)
    ///   - [`parsed_header(Option<structured_encryption::types::ParsedHeader>)`](crate::operation::decrypt_structure::DecryptStructureOutput::parsed_header): (undocumented)
    ///   - [`plaintext_structure(Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>)`](crate::operation::decrypt_structure::DecryptStructureOutput::plaintext_structure): (undocumented)
    /// - On failure, responds with [`SdkError<DecryptStructureError>`](crate::operation::decrypt_structure::DecryptStructureError)
    pub fn decrypt_structure(&self) -> crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder {
        crate::operation::decrypt_structure::builders::DecryptStructureFluentBuilder::new(self.clone())
    }
}
