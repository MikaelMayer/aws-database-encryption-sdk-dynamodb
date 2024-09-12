// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableKeyRotationRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub rotation_period_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl EnableKeyRotationRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn rotation_period_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.rotation_period_in_days
}
}
impl EnableKeyRotationRequest {
    /// Creates a new builder-style object to manufacture [`EnableKeyRotationRequest`](crate::operation::enable_key_rotation::builders::EnableKeyRotationRequest).
    pub fn builder() -> crate::operation::enable_key_rotation::builders::EnableKeyRotationRequestBuilder {
        crate::operation::enable_key_rotation::builders::EnableKeyRotationRequestBuilder::default()
    }
}

/// A builder for [`EnableKeyRotationRequest`](crate::operation::operation::EnableKeyRotationRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnableKeyRotationRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) rotation_period_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl EnableKeyRotationRequestBuilder {
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
pub fn rotation_period_in_days(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.rotation_period_in_days = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_rotation_period_in_days(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.rotation_period_in_days = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_rotation_period_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.rotation_period_in_days
}
    /// Consumes the builder and constructs a [`EnableKeyRotationRequest`](crate::operation::operation::EnableKeyRotationRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_key_rotation::EnableKeyRotationRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_key_rotation::EnableKeyRotationRequest {
            key_id: self.key_id,
rotation_period_in_days: self.rotation_period_in_days,
        })
    }
}
