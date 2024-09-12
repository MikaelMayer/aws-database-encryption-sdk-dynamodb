// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateDataKeyWithoutPlaintext`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`key_spec(impl Into<Option<kms::types::DataKeySpec>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::key_spec) / [`set_key_spec(Option<kms::types::DataKeySpec>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_key_spec): (undocumented)<br>
    ///   - [`number_of_bytes(impl Into<Option<::std::primitive::i32>>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<::std::primitive::i32>)`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::set_number_of_bytes): (undocumented)<br>
    /// - On success, responds with [`GenerateDataKeyWithoutPlaintextResponse`](crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse) with field(s):
    ///   - [`ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse::ciphertext_blob): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse::key_id): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateDataKeyWithoutPlaintextError>`](crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextError)
    pub fn generate_data_key_without_plaintext(&self) -> crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder {
        crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextFluentBuilder::new(self.clone())
    }
}
