// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateDataKeyPairWithoutPlaintext`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`key_pair_spec(impl Into<Option<kms::types::DataKeyPairSpec>>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::key_pair_spec) / [`set_key_pair_spec(Option<kms::types::DataKeyPairSpec>)`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::set_key_pair_spec): (undocumented)<br>
    /// - On success, responds with [`GenerateDataKeyPairWithoutPlaintextResponse`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse::key_id): (undocumented)
    ///   - [`key_pair_spec(Option<kms::types::DataKeyPairSpec>)`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse::key_pair_spec): (undocumented)
    ///   - [`private_key_ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse::private_key_ciphertext_blob): (undocumented)
    ///   - [`public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateDataKeyPairWithoutPlaintextError>`](crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextError)
    pub fn generate_data_key_pair_without_plaintext(&self) -> crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder {
        crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextFluentBuilder::new(self.clone())
    }
}
