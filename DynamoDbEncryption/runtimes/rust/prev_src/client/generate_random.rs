// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateRandom`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    ///   - [`number_of_bytes(impl Into<Option<::std::primitive::i32>>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::number_of_bytes) / [`set_number_of_bytes(Option<::std::primitive::i32>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_number_of_bytes): (undocumented)<br>
    ///   - [`recipient(impl Into<Option<kms::types::RecipientInfo>>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::recipient) / [`set_recipient(Option<kms::types::RecipientInfo>)`](crate::operation::generate_random::builders::GenerateRandomFluentBuilder::set_recipient): (undocumented)<br>
    /// - On success, responds with [`GenerateRandomResponse`](crate::operation::generate_random::GenerateRandomResponse) with field(s):
    ///   - [`ciphertext_for_recipient(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_random::GenerateRandomResponse::ciphertext_for_recipient): (undocumented)
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_random::GenerateRandomResponse::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateRandomError>`](crate::operation::generate_random::GenerateRandomError)
    pub fn generate_random(&self) -> crate::operation::generate_random::builders::GenerateRandomFluentBuilder {
        crate::operation::generate_random::builders::GenerateRandomFluentBuilder::new(self.clone())
    }
}
