// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::primitives::client::Client {
    /// Constructs a fluent builder for the [`CompressPublicKey`](crate::primitives::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecc_curve(impl Into<Option<crate::primitives::types::EcdhCurveSpec>>)`](crate::primitives::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::types::EcdhCurveSpec>)`](crate::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder::set_ecc_curve): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<crate::primitives::types::EccPublicKey>>)`](crate::primitives::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder::public_key) / [`set_public_key(Option<crate::types::EccPublicKey>)`](crate::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`CompressPublicKeyOutput`](crate::primitives::operation::compress_public_key::CompressPublicKeyOutput) with field(s):
    ///   - [`compressed_public_key(Option<::aws_smithy_types::Blob>)`](crate::primitives::operation::compress_public_key::CompressPublicKeyOutput::compressed_public_key): (undocumented)
    /// - On failure, responds with [`SdkError<CompressPublicKeyError>`](crate::primitives::operation::compress_public_key::CompressPublicKeyError)
    pub fn compress_public_key(&self) -> crate::primitives::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder {
        crate::primitives::operation::compress_public_key::builders::CompressPublicKeyFluentBuilder::new(self.clone())
    }
}