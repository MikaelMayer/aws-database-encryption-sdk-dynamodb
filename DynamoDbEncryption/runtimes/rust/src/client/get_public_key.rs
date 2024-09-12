// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetPublicKey`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`GetPublicKeyResponse`](crate::operation::get_public_key::GetPublicKeyResponse) with field(s):
    ///   - [`customer_master_key_spec(Option<kms::types::CustomerMasterKeySpec>)`](crate::operation::get_public_key::GetPublicKeyResponse::customer_master_key_spec): (undocumented)
    ///   - [`encryption_algorithms(Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyResponse::encryption_algorithms): (undocumented)
    ///   - [`key_agreement_algorithms(Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyResponse::key_agreement_algorithms): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::get_public_key::GetPublicKeyResponse::key_id): (undocumented)
    ///   - [`key_spec(Option<kms::types::KeySpec>)`](crate::operation::get_public_key::GetPublicKeyResponse::key_spec): (undocumented)
    ///   - [`key_usage(Option<kms::types::KeyUsageType>)`](crate::operation::get_public_key::GetPublicKeyResponse::key_usage): (undocumented)
    ///   - [`public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::get_public_key::GetPublicKeyResponse::public_key): (undocumented)
    ///   - [`signing_algorithms(Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyResponse::signing_algorithms): (undocumented)
    /// - On failure, responds with [`SdkError<GetPublicKeyError>`](crate::operation::get_public_key::GetPublicKeyError)
    pub fn get_public_key(&self) -> crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder {
        crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::new(self.clone())
    }
}
