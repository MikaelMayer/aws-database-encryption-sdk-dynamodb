// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`Sign`](crate::operation::sign::builders::SignFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::sign::builders::SignFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::sign::builders::SignFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::sign::builders::SignFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::sign::builders::SignFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::sign::builders::SignFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::sign::builders::SignFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::sign::builders::SignFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::sign::builders::SignFluentBuilder::set_message): (undocumented)<br>
    ///   - [`message_type(impl Into<Option<kms::types::MessageType>>)`](crate::operation::sign::builders::SignFluentBuilder::message_type) / [`set_message_type(Option<kms::types::MessageType>)`](crate::operation::sign::builders::SignFluentBuilder::set_message_type): (undocumented)<br>
    ///   - [`signing_algorithm(impl Into<Option<kms::types::SigningAlgorithmSpec>>)`](crate::operation::sign::builders::SignFluentBuilder::signing_algorithm) / [`set_signing_algorithm(Option<kms::types::SigningAlgorithmSpec>)`](crate::operation::sign::builders::SignFluentBuilder::set_signing_algorithm): (undocumented)<br>
    /// - On success, responds with [`SignResponse`](crate::operation::sign::SignResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::sign::SignResponse::key_id): (undocumented)
    ///   - [`signature(Option<::aws_smithy_types::Blob>)`](crate::operation::sign::SignResponse::signature): (undocumented)
    ///   - [`signing_algorithm(Option<kms::types::SigningAlgorithmSpec>)`](crate::operation::sign::SignResponse::signing_algorithm): (undocumented)
    /// - On failure, responds with [`SdkError<SignError>`](crate::operation::sign::SignError)
    pub fn sign(&self) -> crate::operation::sign::builders::SignFluentBuilder {
        crate::operation::sign::builders::SignFluentBuilder::new(self.clone())
    }
}
