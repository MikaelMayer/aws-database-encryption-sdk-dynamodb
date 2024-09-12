// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateDataKeyPair`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`key_pair_spec(impl Into<Option<kms::types::DataKeyPairSpec>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::key_pair_spec) / [`set_key_pair_spec(Option<kms::types::DataKeyPairSpec>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_key_pair_spec): (undocumented)<br>
    ///   - [`recipient(impl Into<Option<kms::types::RecipientInfo>>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::recipient) / [`set_recipient(Option<kms::types::RecipientInfo>)`](crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::set_recipient): (undocumented)<br>
    /// - On success, responds with [`GenerateDataKeyPairResponse`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse) with field(s):
    ///   - [`ciphertext_for_recipient(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::ciphertext_for_recipient): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::key_id): (undocumented)
    ///   - [`key_pair_spec(Option<kms::types::DataKeyPairSpec>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::key_pair_spec): (undocumented)
    ///   - [`private_key_ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::private_key_ciphertext_blob): (undocumented)
    ///   - [`private_key_plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::private_key_plaintext): (undocumented)
    ///   - [`public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_data_key_pair::GenerateDataKeyPairResponse::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateDataKeyPairError>`](crate::operation::generate_data_key_pair::GenerateDataKeyPairError)
    pub fn generate_data_key_pair(&self) -> crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder {
        crate::operation::generate_data_key_pair::builders::GenerateDataKeyPairFluentBuilder::new(self.clone())
    }
}
