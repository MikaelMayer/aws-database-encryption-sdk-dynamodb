// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SignRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub message: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub message_type: ::std::option::Option<kms::types::MessageType>,
#[allow(missing_docs)] // documentation missing in model
pub signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl SignRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
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
pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.message
}
#[allow(missing_docs)] // documentation missing in model
pub fn message_type(&self) -> &::std::option::Option<kms::types::MessageType> {
    &self.message_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn signing_algorithm(&self) -> &::std::option::Option<kms::types::SigningAlgorithmSpec> {
    &self.signing_algorithm
}
}
impl SignRequest {
    /// Creates a new builder-style object to manufacture [`SignRequest`](crate::operation::sign::builders::SignRequest).
    pub fn builder() -> crate::operation::sign::builders::SignRequestBuilder {
        crate::operation::sign::builders::SignRequestBuilder::default()
    }
}

/// A builder for [`SignRequest`](crate::operation::operation::SignRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SignRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) message_type: ::std::option::Option<kms::types::MessageType>,
pub(crate) signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl SignRequestBuilder {
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
pub fn message(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.message = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.message = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.message
}
#[allow(missing_docs)] // documentation missing in model
pub fn message_type(mut self, input: impl ::std::convert::Into<kms::types::MessageType>) -> Self {
    self.message_type = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_message_type(mut self, input: ::std::option::Option<kms::types::MessageType>) -> Self {
    self.message_type = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_message_type(&self) -> &::std::option::Option<kms::types::MessageType> {
    &self.message_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn signing_algorithm(mut self, input: impl ::std::convert::Into<kms::types::SigningAlgorithmSpec>) -> Self {
    self.signing_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signing_algorithm(mut self, input: ::std::option::Option<kms::types::SigningAlgorithmSpec>) -> Self {
    self.signing_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signing_algorithm(&self) -> &::std::option::Option<kms::types::SigningAlgorithmSpec> {
    &self.signing_algorithm
}
    /// Consumes the builder and constructs a [`SignRequest`](crate::operation::operation::SignRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::sign::SignRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::sign::SignRequest {
            dry_run: self.dry_run,
grant_tokens: self.grant_tokens,
key_id: self.key_id,
message: self.message,
message_type: self.message_type,
signing_algorithm: self.signing_algorithm,
        })
    }
}
