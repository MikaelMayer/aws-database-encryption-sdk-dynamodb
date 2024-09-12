// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetKeyRotationStatusRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl GetKeyRotationStatusRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl GetKeyRotationStatusRequest {
    /// Creates a new builder-style object to manufacture [`GetKeyRotationStatusRequest`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusRequest).
    pub fn builder() -> crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusRequestBuilder {
        crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusRequestBuilder::default()
    }
}

/// A builder for [`GetKeyRotationStatusRequest`](crate::operation::operation::GetKeyRotationStatusRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetKeyRotationStatusRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl GetKeyRotationStatusRequestBuilder {
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
    /// Consumes the builder and constructs a [`GetKeyRotationStatusRequest`](crate::operation::operation::GetKeyRotationStatusRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_key_rotation_status::GetKeyRotationStatusRequest {
            key_id: self.key_id,
        })
    }
}
