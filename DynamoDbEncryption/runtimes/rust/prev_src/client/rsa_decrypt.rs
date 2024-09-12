// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`RSADecrypt`](crate::operation::rsa_decrypt::builders::RsaDecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cipher_text(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::cipher_text) / [`set_cipher_text(Option<::aws_smithy_types::Blob>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::set_cipher_text): (undocumented)<br>
    ///   - [`padding(impl Into<Option<primitives::types::RsaPaddingMode>>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::padding) / [`set_padding(Option<primitives::types::RsaPaddingMode>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::set_padding): (undocumented)<br>
    ///   - [`private_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::private_key) / [`set_private_key(Option<::aws_smithy_types::Blob>)`](crate::operation::rsa_decrypt::builders::RSADecryptFluentBuilder::set_private_key): (undocumented)<br>
    /// - On success, responds with [`RsaDecryptOutput`](crate::operation::rsa_decrypt::RsaDecryptOutput) with field(s):
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::rsa_decrypt::RSADecryptOutput::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<RsaDecryptError>`](crate::operation::rsa_decrypt::RsaDecryptError)
    pub fn rsa_decrypt(&self) -> crate::operation::rsa_decrypt::builders::RsaDecryptFluentBuilder {
        crate::operation::rsa_decrypt::builders::RsaDecryptFluentBuilder::new(self.clone())
    }
}
