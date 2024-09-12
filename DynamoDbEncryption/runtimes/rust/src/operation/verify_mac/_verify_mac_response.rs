// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifyMacResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub mac_valid: ::std::option::Option<::std::primitive::bool>,
}
impl VerifyMacResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn mac_algorithm(&self) -> &::std::option::Option<kms::types::MacAlgorithmSpec> {
    &self.mac_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn mac_valid(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.mac_valid
}
}
impl VerifyMacResponse {
    /// Creates a new builder-style object to manufacture [`VerifyMacResponse`](crate::operation::verify_mac::builders::VerifyMacResponse).
    pub fn builder() -> crate::operation::verify_mac::builders::VerifyMacResponseBuilder {
        crate::operation::verify_mac::builders::VerifyMacResponseBuilder::default()
    }
}

/// A builder for [`VerifyMacResponse`](crate::operation::operation::VerifyMacResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VerifyMacResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
pub(crate) mac_valid: ::std::option::Option<::std::primitive::bool>,
}
impl VerifyMacResponseBuilder {
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
pub fn mac_valid(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.mac_valid = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_mac_valid(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.mac_valid = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_mac_valid(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.mac_valid
}
    /// Consumes the builder and constructs a [`VerifyMacResponse`](crate::operation::operation::VerifyMacResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_mac::VerifyMacResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::verify_mac::VerifyMacResponse {
            key_id: self.key_id,
mac_algorithm: self.mac_algorithm,
mac_valid: self.mac_valid,
        })
    }
}
