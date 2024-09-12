// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateMacResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub mac: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
}
impl GenerateMacResponse {
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
}
impl GenerateMacResponse {
    /// Creates a new builder-style object to manufacture [`GenerateMacResponse`](crate::operation::generate_mac::builders::GenerateMacResponse).
    pub fn builder() -> crate::operation::generate_mac::builders::GenerateMacResponseBuilder {
        crate::operation::generate_mac::builders::GenerateMacResponseBuilder::default()
    }
}

/// A builder for [`GenerateMacResponse`](crate::operation::operation::GenerateMacResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateMacResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) mac: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) mac_algorithm: ::std::option::Option<kms::types::MacAlgorithmSpec>,
}
impl GenerateMacResponseBuilder {
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
    /// Consumes the builder and constructs a [`GenerateMacResponse`](crate::operation::operation::GenerateMacResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_mac::GenerateMacResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_mac::GenerateMacResponse {
            key_id: self.key_id,
mac: self.mac,
mac_algorithm: self.mac_algorithm,
        })
    }
}
