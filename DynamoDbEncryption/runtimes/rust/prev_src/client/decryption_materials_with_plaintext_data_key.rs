// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DecryptionMaterialsWithPlaintextDataKey`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`algorithm_suite(impl Into<Option<material_providers::types::AlgorithmSuiteInfo>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::algorithm_suite) / [`set_algorithm_suite(Option<material_providers::types::AlgorithmSuiteInfo>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_algorithm_suite): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`plaintext_data_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::plaintext_data_key) / [`set_plaintext_data_key(Option<::aws_smithy_types::Blob>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_plaintext_data_key): (undocumented)<br>
    ///   - [`required_encryption_context_keys(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::required_encryption_context_keys) / [`set_required_encryption_context_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_required_encryption_context_keys): (undocumented)<br>
    ///   - [`symmetric_signing_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::symmetric_signing_key) / [`set_symmetric_signing_key(Option<::aws_smithy_types::Blob>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_symmetric_signing_key): (undocumented)<br>
    ///   - [`verification_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::verification_key) / [`set_verification_key(Option<::aws_smithy_types::Blob>)`](crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::set_verification_key): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::decryption_materials_with_plaintext_data_key::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DecryptionMaterialsWithPlaintextDataKeyError>`](crate::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterialsWithPlaintextDataKeyError)
    pub fn decryption_materials_with_plaintext_data_key(&self) -> crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder {
        crate::operation::decryption_materials_with_plaintext_data_key::builders::DecryptionMaterialsWithPlaintextDataKeyFluentBuilder::new(self.clone())
    }
}