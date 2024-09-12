// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ReEncrypt`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ciphertext_blob(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::ciphertext_blob) / [`set_ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_ciphertext_blob): (undocumented)<br>
    ///   - [`destination_encryption_algorithm(impl Into<Option<kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_encryption_algorithm) / [`set_destination_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_encryption_algorithm): (undocumented)<br>
    ///   - [`destination_encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_encryption_context) / [`set_destination_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_encryption_context): (undocumented)<br>
    ///   - [`destination_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::destination_key_id) / [`set_destination_key_id(Option<::std::string::String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_destination_key_id): (undocumented)<br>
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`source_encryption_algorithm(impl Into<Option<kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_encryption_algorithm) / [`set_source_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_encryption_algorithm): (undocumented)<br>
    ///   - [`source_encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_encryption_context) / [`set_source_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_encryption_context): (undocumented)<br>
    ///   - [`source_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::source_key_id) / [`set_source_key_id(Option<::std::string::String>)`](crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::set_source_key_id): (undocumented)<br>
    /// - On success, responds with [`ReEncryptResponse`](crate::operation::re_encrypt::ReEncryptResponse) with field(s):
    ///   - [`ciphertext_blob(Option<::aws_smithy_types::Blob>)`](crate::operation::re_encrypt::ReEncryptResponse::ciphertext_blob): (undocumented)
    ///   - [`destination_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::ReEncryptResponse::destination_encryption_algorithm): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::re_encrypt::ReEncryptResponse::key_id): (undocumented)
    ///   - [`source_encryption_algorithm(Option<kms::types::EncryptionAlgorithmSpec>)`](crate::operation::re_encrypt::ReEncryptResponse::source_encryption_algorithm): (undocumented)
    ///   - [`source_key_id(Option<::std::string::String>)`](crate::operation::re_encrypt::ReEncryptResponse::source_key_id): (undocumented)
    /// - On failure, responds with [`SdkError<ReEncryptError>`](crate::operation::re_encrypt::ReEncryptError)
    pub fn re_encrypt(&self) -> crate::operation::re_encrypt::builders::ReEncryptFluentBuilder {
        crate::operation::re_encrypt::builders::ReEncryptFluentBuilder::new(self.clone())
    }
}
