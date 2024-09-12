// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifyMacRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub mac: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl VerifyMacRequest {
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
pub fn mac(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.mac
}
#[allow(missing_docs)] // documentation missing in model
pub fn mac_algorithm(&self) -> &::std::option::Option<kms::types::MacAlgorithmSpec> {
    &self.mac_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.message
}
}
impl VerifyMacRequest {
    /// Creates a new builder-style object to manufacture [`VerifyMacRequest`](crate::operation::verify_mac::builders::VerifyMacRequest).
    pub fn builder() -> crate::operation::verify_mac::builders::VerifyMacRequestBuilder {
        crate::operation::verify_mac::builders::VerifyMacRequestBuilder::default()
    }
}

/// A builder for [`VerifyMacRequest`](crate::operation::operation::VerifyMacRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VerifyMacRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) mac: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
pub(crate) message: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl VerifyMacRequestBuilder {
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
pub fn mac(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.mac = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_mac(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.mac = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_mac(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.mac
}
#[allow(missing_docs)] // documentation missing in model
pub fn mac_algorithm(mut self, input: impl ::std::convert::Into<kms::types::MacAlgorithmSpec>) -> Self {
    self.mac_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_mac_algorithm(mut self, input: ::std::option::Option<kms::types::MacAlgorithmSpec>) -> Self {
    self.mac_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_mac_algorithm(&self) -> &::std::option::Option<kms::types::MacAlgorithmSpec> {
    &self.mac_algorithm
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
    /// Consumes the builder and constructs a [`VerifyMacRequest`](crate::operation::operation::VerifyMacRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_mac::VerifyMacRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::verify_mac::VerifyMacRequest {
            dry_run: self.dry_run,
grant_tokens: self.grant_tokens,
key_id: self.key_id,
mac: self.mac,
mac_algorithm: self.mac_algorithm,
message: self.message,
        })
    }
}
