// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableKeyRotationRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl DisableKeyRotationRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl DisableKeyRotationRequest {
    /// Creates a new builder-style object to manufacture [`DisableKeyRotationRequest`](crate::operation::disable_key_rotation::builders::DisableKeyRotationRequest).
    pub fn builder() -> crate::operation::disable_key_rotation::builders::DisableKeyRotationRequestBuilder {
        crate::operation::disable_key_rotation::builders::DisableKeyRotationRequestBuilder::default()
    }
}

/// A builder for [`DisableKeyRotationRequest`](crate::operation::operation::DisableKeyRotationRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisableKeyRotationRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl DisableKeyRotationRequestBuilder {
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
    /// Consumes the builder and constructs a [`DisableKeyRotationRequest`](crate::operation::operation::DisableKeyRotationRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_key_rotation::DisableKeyRotationRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disable_key_rotation::DisableKeyRotationRequest {
            key_id: self.key_id,
        })
    }
}