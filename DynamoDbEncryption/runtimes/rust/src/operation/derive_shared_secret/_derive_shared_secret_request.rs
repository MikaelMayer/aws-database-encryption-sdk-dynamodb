// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeriveSharedSecretRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_agreement_algorithm: ::std::option::Option<kms::types::KeyAgreementAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl DeriveSharedSecretRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
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
pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
}
impl DeriveSharedSecretRequest {
    /// Creates a new builder-style object to manufacture [`DeriveSharedSecretRequest`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretRequest).
    pub fn builder() -> crate::operation::derive_shared_secret::builders::DeriveSharedSecretRequestBuilder {
        crate::operation::derive_shared_secret::builders::DeriveSharedSecretRequestBuilder::default()
    }
}

/// A builder for [`DeriveSharedSecretRequest`](crate::operation::operation::DeriveSharedSecretRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeriveSharedSecretRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_agreement_algorithm: ::std::option::Option<kms::types::KeyAgreementAlgorithmSpec>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl DeriveSharedSecretRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.dry_run = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_dry_run(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.dry_run = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.grant_tokens = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.grant_tokens = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
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
pub fn public_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.public_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.public_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(mut self, input: impl ::std::convert::Into<kms::types::RecipientInfo>) -> Self {
    self.recipient = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_recipient(mut self, input: ::std::option::Option<kms::types::RecipientInfo>) -> Self {
    self.recipient = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
    /// Consumes the builder and constructs a [`DeriveSharedSecretRequest`](crate::operation::operation::DeriveSharedSecretRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::derive_shared_secret::DeriveSharedSecretRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::derive_shared_secret::DeriveSharedSecretRequest {
            dry_run: self.dry_run,
grant_tokens: self.grant_tokens,
key_agreement_algorithm: self.key_agreement_algorithm,
key_id: self.key_id,
public_key: self.public_key,
recipient: self.recipient,
        })
    }
}
