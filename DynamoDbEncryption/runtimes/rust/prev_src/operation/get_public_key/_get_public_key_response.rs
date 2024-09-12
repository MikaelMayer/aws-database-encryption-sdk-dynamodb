// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPublicKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub customer_master_key_spec: ::std::option::Option<kms::types::CustomerMasterKeySpec>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>>,
#[allow(missing_docs)] // documentation missing in model
pub key_agreement_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_spec: ::std::option::Option<kms::types::KeySpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_usage: ::std::option::Option<kms::types::KeyUsageType>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub signing_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>>,
}
impl GetPublicKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn customer_master_key_spec(&self) -> &::std::option::Option<kms::types::CustomerMasterKeySpec> {
    &self.customer_master_key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>> {
    &self.encryption_algorithms
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>> {
    &self.key_agreement_algorithms
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_spec(&self) -> &::std::option::Option<kms::types::KeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_usage(&self) -> &::std::option::Option<kms::types::KeyUsageType> {
    &self.key_usage
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn signing_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>> {
    &self.signing_algorithms
}
}
impl GetPublicKeyResponse {
    /// Creates a new builder-style object to manufacture [`GetPublicKeyResponse`](crate::operation::get_public_key::builders::GetPublicKeyResponse).
    pub fn builder() -> crate::operation::get_public_key::builders::GetPublicKeyResponseBuilder {
        crate::operation::get_public_key::builders::GetPublicKeyResponseBuilder::default()
    }
}

/// A builder for [`GetPublicKeyResponse`](crate::operation::operation::GetPublicKeyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPublicKeyResponseBuilder {
    pub(crate) customer_master_key_spec: ::std::option::Option<kms::types::CustomerMasterKeySpec>,
pub(crate) encryption_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>>,
pub(crate) key_agreement_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_spec: ::std::option::Option<kms::types::KeySpec>,
pub(crate) key_usage: ::std::option::Option<kms::types::KeyUsageType>,
pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) signing_algorithms: ::std::option::Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>>,
}
impl GetPublicKeyResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn customer_master_key_spec(mut self, input: impl ::std::convert::Into<kms::types::CustomerMasterKeySpec>) -> Self {
    self.customer_master_key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_customer_master_key_spec(mut self, input: ::std::option::Option<kms::types::CustomerMasterKeySpec>) -> Self {
    self.customer_master_key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_customer_master_key_spec(&self) -> &::std::option::Option<kms::types::CustomerMasterKeySpec> {
    &self.customer_master_key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_algorithms(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>>) -> Self {
    self.encryption_algorithms = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>>) -> Self {
    self.encryption_algorithms = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::EncryptionAlgorithmSpec>> {
    &self.encryption_algorithms
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_agreement_algorithms(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>>) -> Self {
    self.key_agreement_algorithms = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_agreement_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>>) -> Self {
    self.key_agreement_algorithms = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_agreement_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::KeyAgreementAlgorithmSpec>> {
    &self.key_agreement_algorithms
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
pub fn key_spec(mut self, input: impl ::std::convert::Into<kms::types::KeySpec>) -> Self {
    self.key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_spec(mut self, input: ::std::option::Option<kms::types::KeySpec>) -> Self {
    self.key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_spec(&self) -> &::std::option::Option<kms::types::KeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_usage(mut self, input: impl ::std::convert::Into<kms::types::KeyUsageType>) -> Self {
    self.key_usage = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_usage(mut self, input: ::std::option::Option<kms::types::KeyUsageType>) -> Self {
    self.key_usage = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_usage(&self) -> &::std::option::Option<kms::types::KeyUsageType> {
    &self.key_usage
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
pub fn signing_algorithms(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::SigningAlgorithmSpec>>) -> Self {
    self.signing_algorithms = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signing_algorithms(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>>) -> Self {
    self.signing_algorithms = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signing_algorithms(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::SigningAlgorithmSpec>> {
    &self.signing_algorithms
}
    /// Consumes the builder and constructs a [`GetPublicKeyResponse`](crate::operation::operation::GetPublicKeyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_public_key::GetPublicKeyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_public_key::GetPublicKeyResponse {
            customer_master_key_spec: self.customer_master_key_spec,
encryption_algorithms: self.encryption_algorithms,
key_agreement_algorithms: self.key_agreement_algorithms,
key_id: self.key_id,
key_spec: self.key_spec,
key_usage: self.key_usage,
public_key: self.public_key,
signing_algorithms: self.signing_algorithms,
        })
    }
}
