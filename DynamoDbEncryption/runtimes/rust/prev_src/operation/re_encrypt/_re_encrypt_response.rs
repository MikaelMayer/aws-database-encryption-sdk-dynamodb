// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReEncryptResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub destination_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub source_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub source_key_id: ::std::option::Option<::std::string::String>,
}
impl ReEncryptResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.destination_encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.source_encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_key_id
}
}
impl ReEncryptResponse {
    /// Creates a new builder-style object to manufacture [`ReEncryptResponse`](crate::operation::re_encrypt::builders::ReEncryptResponse).
    pub fn builder() -> crate::operation::re_encrypt::builders::ReEncryptResponseBuilder {
        crate::operation::re_encrypt::builders::ReEncryptResponseBuilder::default()
    }
}

/// A builder for [`ReEncryptResponse`](crate::operation::operation::ReEncryptResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReEncryptResponseBuilder {
    pub(crate) ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) destination_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) source_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) source_key_id: ::std::option::Option<::std::string::String>,
}
impl ReEncryptResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_blob = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ciphertext_blob(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_blob = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.destination_encryption_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.destination_encryption_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.destination_encryption_algorithm
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
pub fn source_encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.source_encryption_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.source_encryption_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.source_encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.source_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.source_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_key_id
}
    /// Consumes the builder and constructs a [`ReEncryptResponse`](crate::operation::operation::ReEncryptResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::re_encrypt::ReEncryptResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::re_encrypt::ReEncryptResponse {
            ciphertext_blob: self.ciphertext_blob,
destination_encryption_algorithm: self.destination_encryption_algorithm,
key_id: self.key_id,
source_encryption_algorithm: self.source_encryption_algorithm,
source_key_id: self.source_key_id,
        })
    }
}
