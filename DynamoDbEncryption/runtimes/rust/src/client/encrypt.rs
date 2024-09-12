// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Encrypt`](crate::operation::encrypt::builders::EncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`encryption_algorithm(impl Into<Option<kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_algorithm) / [`set_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_algorithm): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_context): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`plaintext(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::plaintext) / [`set_plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_plaintext): (undocumented)<br>
    /// - On success, responds with [`EncryptResponse`](crate::operation::encrypt::EncryptResponse) with field(s):
    ///   - [`ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::encrypt::EncryptResponse::ciphertext_blob): (undocumented)
    ///   - [`encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::encrypt::EncryptResponse::encryption_algorithm): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::encrypt::EncryptResponse::key_id): (undocumented)
    /// - On failure, responds with [`SdkError<EncryptError>`](crate::operation::encrypt::EncryptError)
    pub fn encrypt(&self) -> crate::operation::encrypt::builders::EncryptFluentBuilder {
        crate::operation::encrypt::builders::EncryptFluentBuilder::new(self.clone())
    }
}
