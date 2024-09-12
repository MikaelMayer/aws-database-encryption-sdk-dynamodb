// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GenerateMac`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`mac_algorithm(impl Into<Option<kms::types::MacAlgorithmSpec>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::mac_algorithm) / [`set_mac_algorithm(Option<kms::types::MacAlgorithmSpec>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_mac_algorithm): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_mac::builders::GenerateMacFluentBuilder::set_message): (undocumented)<br>
    /// - On success, responds with [`GenerateMacResponse`](crate::operation::generate_mac::GenerateMacResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::generate_mac::GenerateMacResponse::key_id): (undocumented)
    ///   - [`mac(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_mac::GenerateMacResponse::mac): (undocumented)
    ///   - [`mac_algorithm(Option<kms::types::MacAlgorithmSpec>)`](crate::operation::generate_mac::GenerateMacResponse::mac_algorithm): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateMacError>`](crate::operation::generate_mac::GenerateMacError)
    pub fn generate_mac(&self) -> crate::operation::generate_mac::builders::GenerateMacFluentBuilder {
        crate::operation::generate_mac::builders::GenerateMacFluentBuilder::new(self.clone())
    }
}
