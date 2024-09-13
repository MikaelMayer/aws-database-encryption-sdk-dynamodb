// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`HMac`](crate::operation::h_mac::builders::HMacFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<primitives::types::DigestAlgorithm>>)`](crate::operation::h_mac::builders::HMacFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<primitives::types::DigestAlgorithm>)`](crate::operation::h_mac::builders::HMacFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::h_mac::builders::HMacFluentBuilder::key) / [`set_key(Option<::aws_smithy_types::Blob>)`](crate::operation::h_mac::builders::HMacFluentBuilder::set_key): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::h_mac::builders::HMacFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::h_mac::builders::HMacFluentBuilder::set_message): (undocumented)<br>
    /// - On success, responds with [`HMacOutput`](crate::operation::h_mac::HMacOutput) with field(s):
    ///   - [`digest(Option<::aws_smithy_types::Blob>)`](crate::operation::h_mac::HMacOutput::digest): (undocumented)
    /// - On failure, responds with [`SdkError<HMacError>`](crate::operation::h_mac::HMacError)
    pub fn h_mac(&self) -> crate::operation::h_mac::builders::HMacFluentBuilder {
        crate::operation::h_mac::builders::HMacFluentBuilder::new(self.clone())
    }
}