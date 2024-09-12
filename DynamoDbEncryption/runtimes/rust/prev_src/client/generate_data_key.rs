// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateDataKey`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`key_spec(impl Into<Option<kms::types::DataKeySpec>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::key_spec) / [`set_key_spec(Option<kms::types::DataKeySpec>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_key_spec): (undocumented)<br>
    ///   - [`number_of_bytes(impl Into<Option<::std::primitive::i32>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<::std::primitive::i32>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_number_of_bytes): (undocumented)<br>
    ///   - [`recipient(impl Into<Option<kms::types::RecipientInfo>>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::recipient) / [`set_recipient(Option<kms::types::RecipientInfo>)`](crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::set_recipient): (undocumented)<br>
    /// - On success, responds with [`GenerateDataKeyResponse`](crate::operation::generate_data_key::GenerateDataKeyResponse) with field(s):
    ///   - [`ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key::GenerateDataKeyResponse::ciphertext_blob): (undocumented)
    ///   - [`ciphertext_for_recipient(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key::GenerateDataKeyResponse::ciphertext_for_recipient): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::generate_data_key::GenerateDataKeyResponse::key_id): (undocumented)
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key::GenerateDataKeyResponse::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateDataKeyError>`](crate::operation::generate_data_key::GenerateDataKeyError)
    pub fn generate_data_key(&self) -> crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder {
        crate::operation::generate_data_key::builders::GenerateDataKeyFluentBuilder::new(self.clone())
    }
}
