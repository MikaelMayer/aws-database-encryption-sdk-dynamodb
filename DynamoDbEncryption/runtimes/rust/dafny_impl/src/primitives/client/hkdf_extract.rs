// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::primitives::client::Client {
    /// Constructs a fluent builder for the [`HkdfExtract`](crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::primitives::types::DigestAlgorithm>>)`](crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::types::DigestAlgorithm>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`ikm(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::ikm) / [`set_ikm(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_ikm): (undocumented)<br>
    ///   - [`salt(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::salt) / [`set_salt(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_salt): (undocumented)<br>
    /// - On success, responds with [`HkdfExtractOutput`](crate::primitives::operation::hkdf_extract::HkdfExtractOutput) with field(s):
    ///   - [`prk(Option<::aws_smithy_types::Blob>)`](crate::primitives::operation::hkdf_extract::HkdfExtractOutput::prk): (undocumented)
    /// - On failure, responds with [`SdkError<HkdfExtractError>`](crate::primitives::operation::hkdf_extract::HkdfExtractError)
    pub fn hkdf_extract(&self) -> crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder {
        crate::primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::new(self.clone())
    }
}
