// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl DisableKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl DisableKeyRequest {
    /// Creates a new builder-style object to manufacture [`DisableKeyRequest`](crate::operation::disable_key::builders::DisableKeyRequest).
    pub fn builder() -> crate::operation::disable_key::builders::DisableKeyRequestBuilder {
        crate::operation::disable_key::builders::DisableKeyRequestBuilder::default()
    }
}

/// A builder for [`DisableKeyRequest`](crate::operation::operation::DisableKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisableKeyRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl DisableKeyRequestBuilder {
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
    /// Consumes the builder and constructs a [`DisableKeyRequest`](crate::operation::operation::DisableKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_key::DisableKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disable_key::DisableKeyRequest {
            key_id: self.key_id,
        })
    }
}
