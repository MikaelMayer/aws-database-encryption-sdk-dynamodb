// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::primitives::client::Client {
    /// Constructs a fluent builder for the [`RSAEncrypt`](crate::primitives::operation::rsa_encrypt::builders::RsaEncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`padding(impl Into<Option<crate::primitives::types::RsaPaddingMode>>)`](crate::primitives::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::padding) / [`set_padding(Option<crate::types::RsaPaddingMode>)`](crate::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::set_padding): (undocumented)<br>
    ///   - [`plaintext(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::primitives::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::plaintext) / [`set_plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::set_plaintext): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::primitives::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::rsa_encrypt::builders::RSAEncryptFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`RsaEncryptOutput`](crate::primitives::operation::rsa_encrypt::RsaEncryptOutput) with field(s):
    ///   - [`cipher_text(Option<::aws_smithy_types::Blob>)`](crate::primitives::operation::rsa_encrypt::RSAEncryptOutput::cipher_text): (undocumented)
    /// - On failure, responds with [`SdkError<RsaEncryptError>`](crate::primitives::operation::rsa_encrypt::RsaEncryptError)
    pub fn rsa_encrypt(&self) -> crate::primitives::operation::rsa_encrypt::builders::RsaEncryptFluentBuilder {
        crate::primitives::operation::rsa_encrypt::builders::RsaEncryptFluentBuilder::new(self.clone())
    }
}
