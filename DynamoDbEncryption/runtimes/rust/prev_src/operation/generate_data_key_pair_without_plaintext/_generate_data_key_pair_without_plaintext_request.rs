// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateDataKeyPairWithoutPlaintextRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_pair_spec: ::std::option::Option<kms::types::DataKeyPairSpec>,
}
impl GenerateDataKeyPairWithoutPlaintextRequest {
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
pub fn key_pair_spec(&self) -> &::std::option::Option<kms::types::DataKeyPairSpec> {
    &self.key_pair_spec
}
}
impl GenerateDataKeyPairWithoutPlaintextRequest {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyPairWithoutPlaintextRequest`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextRequest).
    pub fn builder() -> crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextRequestBuilder {
        crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextRequestBuilder::default()
    }
}

/// A builder for [`GenerateDataKeyPairWithoutPlaintextRequest`](crate::operation::operation::GenerateDataKeyPairWithoutPlaintextRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateDataKeyPairWithoutPlaintextRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_pair_spec: ::std::option::Option<kms::types::DataKeyPairSpec>,
}
impl GenerateDataKeyPairWithoutPlaintextRequestBuilder {
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
pub fn key_pair_spec(mut self, input: impl ::std::convert::Into<kms::types::DataKeyPairSpec>) -> Self {
    self.key_pair_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_pair_spec(mut self, input: ::std::option::Option<kms::types::DataKeyPairSpec>) -> Self {
    self.key_pair_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_pair_spec(&self) -> &::std::option::Option<kms::types::DataKeyPairSpec> {
    &self.key_pair_spec
}
    /// Consumes the builder and constructs a [`GenerateDataKeyPairWithoutPlaintextRequest`](crate::operation::operation::GenerateDataKeyPairWithoutPlaintextRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextRequest {
            dry_run: self.dry_run,
encryption_context: self.encryption_context,
grant_tokens: self.grant_tokens,
key_id: self.key_id,
key_pair_spec: self.key_pair_spec,
        })
    }
}
