// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateDataKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_spec: ::std::option::Option<kms::types::DataKeySpec>,
#[allow(missing_docs)] // documentation missing in model
pub number_of_bytes: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl GenerateDataKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
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
pub fn key_spec(&self) -> &::std::option::Option<kms::types::DataKeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn number_of_bytes(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.number_of_bytes
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
}
impl GenerateDataKeyRequest {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyRequest`](crate::operation::generate_data_key::builders::GenerateDataKeyRequest).
    pub fn builder() -> crate::operation::generate_data_key::builders::GenerateDataKeyRequestBuilder {
        crate::operation::generate_data_key::builders::GenerateDataKeyRequestBuilder::default()
    }
}

/// A builder for [`GenerateDataKeyRequest`](crate::operation::operation::GenerateDataKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateDataKeyRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_spec: ::std::option::Option<kms::types::DataKeySpec>,
pub(crate) number_of_bytes: ::std::option::Option<::std::primitive::i32>,
pub(crate) recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl GenerateDataKeyRequestBuilder {
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
pub fn key_spec(mut self, input: impl ::std::convert::Into<kms::types::DataKeySpec>) -> Self {
    self.key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_spec(mut self, input: ::std::option::Option<kms::types::DataKeySpec>) -> Self {
    self.key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_spec(&self) -> &::std::option::Option<kms::types::DataKeySpec> {
    &self.key_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn number_of_bytes(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.number_of_bytes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_number_of_bytes(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.number_of_bytes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_number_of_bytes(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.number_of_bytes
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
    /// Consumes the builder and constructs a [`GenerateDataKeyRequest`](crate::operation::operation::GenerateDataKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key::GenerateDataKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_data_key::GenerateDataKeyRequest {
            dry_run: self.dry_run,
encryption_context: self.encryption_context,
grant_tokens: self.grant_tokens,
key_id: self.key_id,
key_spec: self.key_spec,
number_of_bytes: self.number_of_bytes,
recipient: self.recipient,
        })
    }
}