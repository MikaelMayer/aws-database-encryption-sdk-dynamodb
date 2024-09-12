// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeriveSharedSecretResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub key_agreement_algorithm: ::std::option::Option<kms::types::KeyAgreementAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_origin: ::std::option::Option<kms::types::OriginType>,
#[allow(missing_docs)] // documentation missing in model
pub shared_secret: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DeriveSharedSecretResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_algorithm(&self) -> &::std::option::Option<kms::types::KeyAgreementAlgorithmSpec> {
    &self.key_agreement_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_origin(&self) -> &::std::option::Option<kms::types::OriginType> {
    &self.key_origin
}
#[allow(missing_docs)] // documentation missing in model
pub fn shared_secret(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.shared_secret
}
}
impl DeriveSharedSecretResponse {
    /// Creates a new builder-style object to manufacture [`DeriveSharedSecretResponse`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretResponse).
    pub fn builder() -> crate::operation::derive_shared_secret::builders::DeriveSharedSecretResponseBuilder {
        crate::operation::derive_shared_secret::builders::DeriveSharedSecretResponseBuilder::default()
    }
}

/// A builder for [`DeriveSharedSecretResponse`](crate::operation::operation::DeriveSharedSecretResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeriveSharedSecretResponseBuilder {
    pub(crate) ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) key_agreement_algorithm: ::std::option::Option<kms::types::KeyAgreementAlgorithmSpec>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_origin: ::std::option::Option<kms::types::OriginType>,
pub(crate) shared_secret: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DeriveSharedSecretResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_for_recipient = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ciphertext_for_recipient(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_for_recipient = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_algorithm(mut self, input: impl ::std::convert::Into<kms::types::KeyAgreementAlgorithmSpec>) -> Self {
    self.key_agreement_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_agreement_algorithm(mut self, input: ::std::option::Option<kms::types::KeyAgreementAlgorithmSpec>) -> Self {
    self.key_agreement_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_agreement_algorithm(&self) -> &::std::option::Option<kms::types::KeyAgreementAlgorithmSpec> {
    &self.key_agreement_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_origin(mut self, input: impl ::std::convert::Into<kms::types::OriginType>) -> Self {
    self.key_origin = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_origin(mut self, input: ::std::option::Option<kms::types::OriginType>) -> Self {
    self.key_origin = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_origin(&self) -> &::std::option::Option<kms::types::OriginType> {
    &self.key_origin
}
#[allow(missing_docs)] // documentation missing in model
pub fn shared_secret(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.shared_secret = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_shared_secret(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.shared_secret = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_shared_secret(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.shared_secret
}
    /// Consumes the builder and constructs a [`DeriveSharedSecretResponse`](crate::operation::operation::DeriveSharedSecretResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::derive_shared_secret::DeriveSharedSecretResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::derive_shared_secret::DeriveSharedSecretResponse {
            ciphertext_for_recipient: self.ciphertext_for_recipient,
key_agreement_algorithm: self.key_agreement_algorithm,
key_id: self.key_id,
key_origin: self.key_origin,
shared_secret: self.shared_secret,
        })
    }
}
