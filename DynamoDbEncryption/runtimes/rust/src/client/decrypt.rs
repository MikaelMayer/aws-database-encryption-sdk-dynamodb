// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Decrypt`](crate::operation::decrypt::builders::DecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ciphertext_blob(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::ciphertext_blob) / [`set_ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_ciphertext_blob): (undocumented)<br>
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_algorithm(impl Into<Option<kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::encryption_algorithm) / [`set_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_encryption_algorithm): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`recipient(impl Into<Option<kms::types::RecipientInfo>>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::recipient) / [`set_recipient(Option<kms::types::RecipientInfo>)`](crate::operation::decrypt::builders::DecryptFluentBuilder::set_recipient): (undocumented)<br>
    /// - On success, responds with [`DecryptResponse`](crate::operation::decrypt::DecryptResponse) with field(s):
    ///   - [`ciphertext_for_recipient(Option<::aws_smithy_types::Blob>)`](crate::operation::decrypt::DecryptResponse::ciphertext_for_recipient): (undocumented)
    ///   - [`encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::decrypt::DecryptResponse::encryption_algorithm): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::decrypt::DecryptResponse::key_id): (undocumented)
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::decrypt::DecryptResponse::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<DecryptError>`](crate::operation::decrypt::DecryptError)
    pub fn decrypt(&self) -> crate::operation::decrypt::builders::DecryptFluentBuilder {
        crate::operation::decrypt::builders::DecryptFluentBuilder::new(self.clone())
    }
}
