// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecryptResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
}
impl DecryptResponse {
    /// Creates a new builder-style object to manufacture [`DecryptResponse`](crate::operation::decrypt::builders::DecryptResponse).
    pub fn builder() -> crate::operation::decrypt::builders::DecryptResponseBuilder {
        crate::operation::decrypt::builders::DecryptResponseBuilder::default()
    }
}

/// A builder for [`DecryptResponse`](crate::operation::operation::DecryptResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptResponseBuilder {
    pub(crate) ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl DecryptResponseBuilder {
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
pub fn encryption_algorithm(mut self, input: impl ::std::convert::Into<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.encryption_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_algorithm(mut self, input: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>) -> Self {
    self.encryption_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.encryption_algorithm
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
pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
    /// Consumes the builder and constructs a [`DecryptResponse`](crate::operation::operation::DecryptResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::decrypt::DecryptResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::decrypt::DecryptResponse {
            ciphertext_for_recipient: self.ciphertext_for_recipient,
encryption_algorithm: self.encryption_algorithm,
key_id: self.key_id,
plaintext: self.plaintext,
        })
    }
}
