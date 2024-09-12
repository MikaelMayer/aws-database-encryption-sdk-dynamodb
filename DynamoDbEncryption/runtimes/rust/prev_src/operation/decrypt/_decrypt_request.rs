// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecryptRequest {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl DecryptRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
}
impl DecryptRequest {
    /// Creates a new builder-style object to manufacture [`DecryptRequest`](crate::operation::decrypt::builders::DecryptRequest).
    pub fn builder() -> crate::operation::decrypt::builders::DecryptRequestBuilder {
        crate::operation::decrypt::builders::DecryptRequestBuilder::default()
    }
}

/// A builder for [`DecryptRequest`](crate::operation::operation::DecryptRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptRequestBuilder {
    pub(crate) ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl DecryptRequestBuilder {
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
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
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
    /// Consumes the builder and constructs a [`DecryptRequest`](crate::operation::operation::DecryptRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::decrypt::DecryptRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::decrypt::DecryptRequest {
            ciphertext_blob: self.ciphertext_blob,
dry_run: self.dry_run,
encryption_algorithm: self.encryption_algorithm,
encryption_context: self.encryption_context,
grant_tokens: self.grant_tokens,
key_id: self.key_id,
recipient: self.recipient,
        })
    }
}
