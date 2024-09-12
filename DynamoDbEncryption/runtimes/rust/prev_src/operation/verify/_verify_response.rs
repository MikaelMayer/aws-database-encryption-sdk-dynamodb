// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub signature_valid: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl VerifyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn signature_valid(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.signature_valid
}
#[allow(missing_docs)] // documentation missing in model
pub fn signing_algorithm(&self) -> &::std::option::Option<kms::types::SigningAlgorithmSpec> {
    &self.signing_algorithm
}
}
impl VerifyResponse {
    /// Creates a new builder-style object to manufacture [`VerifyResponse`](crate::operation::verify::builders::VerifyResponse).
    pub fn builder() -> crate::operation::verify::builders::VerifyResponseBuilder {
        crate::operation::verify::builders::VerifyResponseBuilder::default()
    }
}

/// A builder for [`VerifyResponse`](crate::operation::operation::VerifyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VerifyResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) signature_valid: ::std::option::Option<::std::primitive::bool>,
pub(crate) signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl VerifyResponseBuilder {
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
pub fn signature_valid(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.signature_valid = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signature_valid(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.signature_valid = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signature_valid(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.signature_valid
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
    /// Consumes the builder and constructs a [`VerifyResponse`](crate::operation::operation::VerifyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify::VerifyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::verify::VerifyResponse {
            key_id: self.key_id,
signature_valid: self.signature_valid,
signing_algorithm: self.signing_algorithm,
        })
    }
}
