// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Verify`](crate::operation::verify::builders::VerifyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::verify::builders::VerifyFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::verify::builders::VerifyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::verify::builders::VerifyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::verify::builders::VerifyFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_message): (undocumented)<br>
    ///   - [`message_type(impl Into<Option<kms::types::MessageType>>)`](crate::operation::verify::builders::VerifyFluentBuilder::message_type) / [`set_message_type(Option<kms::types::MessageType>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_message_type): (undocumented)<br>
    ///   - [`signature(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::verify::builders::VerifyFluentBuilder::signature) / [`set_signature(Option<::aws_smithy_types::Blob>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_signature): (undocumented)<br>
    ///   - [`signing_algorithm(impl Into<Option<kms::types::SigningAlgorithmSpec>>)`](crate::operation::verify::builders::VerifyFluentBuilder::signing_algorithm) / [`set_signing_algorithm(Option<kms::types::SigningAlgorithmSpec>)`](crate::operation::verify::builders::VerifyFluentBuilder::set_signing_algorithm): (undocumented)<br>
    /// - On success, responds with [`VerifyResponse`](crate::operation::verify::VerifyResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::verify::VerifyResponse::key_id): (undocumented)
    ///   - [`signature_valid(Option<::std::primitive::bool>)`](crate::operation::verify::VerifyResponse::signature_valid): (undocumented)
    ///   - [`signing_algorithm(Option<kms::types::SigningAlgorithmSpec>)`](crate::operation::verify::VerifyResponse::signing_algorithm): (undocumented)
    /// - On failure, responds with [`SdkError<VerifyError>`](crate::operation::verify::VerifyError)
    pub fn verify(&self) -> crate::operation::verify::builders::VerifyFluentBuilder {
        crate::operation::verify::builders::VerifyFluentBuilder::new(self.clone())
    }
}
