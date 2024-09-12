// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SignResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub signature: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl SignResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.signature
}
#[allow(missing_docs)] // documentation missing in model
pub fn signing_algorithm(&self) -> &::std::option::Option<kms::types::SigningAlgorithmSpec> {
    &self.signing_algorithm
}
}
impl SignResponse {
    /// Creates a new builder-style object to manufacture [`SignResponse`](crate::operation::sign::builders::SignResponse).
    pub fn builder() -> crate::operation::sign::builders::SignResponseBuilder {
        crate::operation::sign::builders::SignResponseBuilder::default()
    }
}

/// A builder for [`SignResponse`](crate::operation::operation::SignResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SignResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) signature: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) signing_algorithm: ::std::option::Option<kms::types::SigningAlgorithmSpec>,
}
impl SignResponseBuilder {
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
pub fn signature(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.signature = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_signature(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.signature = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_signature(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.signature
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
    /// Consumes the builder and constructs a [`SignResponse`](crate::operation::operation::SignResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::sign::SignResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::sign::SignResponse {
            key_id: self.key_id,
signature: self.signature,
signing_algorithm: self.signing_algorithm,
        })
    }
}
