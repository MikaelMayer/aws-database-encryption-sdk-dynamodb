// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`VerifyMac`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`mac(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::mac) / [`set_mac(Option<::aws_smithy_types::Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_mac): (undocumented)<br>
    ///   - [`mac_algorithm(impl Into<Option<kms::types::MacAlgorithmSpec>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::mac_algorithm) / [`set_mac_algorithm(Option<kms::types::MacAlgorithmSpec>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_mac_algorithm): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::verify_mac::builders::VerifyMacFluentBuilder::set_message): (undocumented)<br>
    /// - On success, responds with [`VerifyMacResponse`](crate::operation::verify_mac::VerifyMacResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::verify_mac::VerifyMacResponse::key_id): (undocumented)
    ///   - [`mac_algorithm(Option<kms::types::MacAlgorithmSpec>)`](crate::operation::verify_mac::VerifyMacResponse::mac_algorithm): (undocumented)
    ///   - [`mac_valid(Option<::std::primitive::bool>)`](crate::operation::verify_mac::VerifyMacResponse::mac_valid): (undocumented)
    /// - On failure, responds with [`SdkError<VerifyMacError>`](crate::operation::verify_mac::VerifyMacError)
    pub fn verify_mac(&self) -> crate::operation::verify_mac::builders::VerifyMacFluentBuilder {
        crate::operation::verify_mac::builders::VerifyMacFluentBuilder::new(self.clone())
    }
}
