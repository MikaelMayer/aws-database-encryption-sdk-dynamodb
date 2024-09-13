// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::primitives::client::Client {
    /// Constructs a fluent builder for the [`GenerateRSAKeyPair`](crate::primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`length_bits(impl Into<Option<::std::primitive::i32>>)`](crate::primitives::operation::generate_rsa_key_pair::builders::GenerateRSAKeyPairFluentBuilder::length_bits) / [`set_length_bits(Option<::std::primitive::i32>)`](crate::operation::generate_rsa_key_pair::builders::GenerateRSAKeyPairFluentBuilder::set_length_bits): (undocumented)<br>
    /// - On success, responds with [`GenerateRsaKeyPairOutput`](crate::primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput) with field(s):
    ///   - [`private_key(Option<crate::primitives::types::RsaPrivateKey>)`](crate::primitives::operation::generate_rsa_key_pair::GenerateRSAKeyPairOutput::private_key): (undocumented)
    ///   - [`public_key(Option<crate::primitives::types::RsaPublicKey>)`](crate::primitives::operation::generate_rsa_key_pair::GenerateRSAKeyPairOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateRsaKeyPairError>`](crate::primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairError)
    pub fn generate_rsa_key_pair(&self) -> crate::primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder {
        crate::primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder::new(self.clone())
    }
}