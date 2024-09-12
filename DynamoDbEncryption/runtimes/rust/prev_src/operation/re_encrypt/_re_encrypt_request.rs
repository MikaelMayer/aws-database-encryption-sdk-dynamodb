// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReEncryptRequest {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub destination_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub destination_encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub destination_key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub source_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub source_encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub source_key_id: ::std::option::Option<::std::string::String>,
}
impl ReEncryptRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.destination_encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.destination_encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.destination_key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_encryption_algorithm(&self) -> &::std::option::Option<kms::types::EncryptionAlgorithmSpec> {
    &self.source_encryption_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.source_encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn source_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.source_key_id
}
}
impl ReEncryptRequest {
    /// Creates a new builder-style object to manufacture [`ReEncryptRequest`](crate::operation::re_encrypt::builders::ReEncryptRequest).
    pub fn builder() -> crate::operation::re_encrypt::builders::ReEncryptRequestBuilder {
        crate::operation::re_encrypt::builders::ReEncryptRequestBuilder::default()
    }
}

/// A builder for [`ReEncryptRequest`](crate::operation::operation::ReEncryptRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReEncryptRequestBuilder {
    pub(crate) ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) destination_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) destination_encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) destination_key_id: ::std::option::Option<::std::string::String>,
pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) source_encryption_algorithm: ::std::option::Option<kms::types::EncryptionAlgorithmSpec>,
pub(crate) source_encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) source_key_id: ::std::option::Option<::std::string::String>,
}
impl ReEncryptRequestBuilder {
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
pub fn destination_encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.destination_encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.destination_encryption_context = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.destination_encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn destination_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.destination_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.destination_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.destination_key_id
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
pub fn source_encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.source_encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_source_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.source_encryption_context = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_source_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.source_encryption_context
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
    /// Consumes the builder and constructs a [`ReEncryptRequest`](crate::operation::operation::ReEncryptRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::re_encrypt::ReEncryptRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::re_encrypt::ReEncryptRequest {
            ciphertext_blob: self.ciphertext_blob,
destination_encryption_algorithm: self.destination_encryption_algorithm,
destination_encryption_context: self.destination_encryption_context,
destination_key_id: self.destination_key_id,
dry_run: self.dry_run,
grant_tokens: self.grant_tokens,
source_encryption_algorithm: self.source_encryption_algorithm,
source_encryption_context: self.source_encryption_context,
source_key_id: self.source_key_id,
        })
    }
}
