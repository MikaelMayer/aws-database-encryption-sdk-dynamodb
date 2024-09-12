// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeriveSharedSecret`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_agreement_algorithm(impl Into<Option<kms::types::KeyAgreementAlgorithmSpec>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::key_agreement_algorithm) / [`set_key_agreement_algorithm(Option<kms::types::KeyAgreementAlgorithmSpec>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_key_agreement_algorithm): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_public_key): (undocumented)<br>
    ///   - [`recipient(impl Into<Option<kms::types::RecipientInfo>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::recipient) / [`set_recipient(Option<kms::types::RecipientInfo>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_recipient): (undocumented)<br>
    /// - On success, responds with [`DeriveSharedSecretResponse`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse) with field(s):
    ///   - [`ciphertext_for_recipient(Option<::aws_smithy_types::Blob>)`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse::ciphertext_for_recipient): (undocumented)
    ///   - [`key_agreement_algorithm(Option<kms::types::KeyAgreementAlgorithmSpec>)`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse::key_agreement_algorithm): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse::key_id): (undocumented)
    ///   - [`key_origin(Option<kms::types::OriginType>)`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse::key_origin): (undocumented)
    ///   - [`shared_secret(Option<::aws_smithy_types::Blob>)`](crate::operation::derive_shared_secret::DeriveSharedSecretResponse::shared_secret): (undocumented)
    /// - On failure, responds with [`SdkError<DeriveSharedSecretError>`](crate::operation::derive_shared_secret::DeriveSharedSecretError)
    pub fn derive_shared_secret(&self) -> crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder {
        crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::new(self.clone())
    }
}
