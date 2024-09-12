// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl EnableKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl EnableKeyRequest {
    /// Creates a new builder-style object to manufacture [`EnableKeyRequest`](crate::operation::enable_key::builders::EnableKeyRequest).
    pub fn builder() -> crate::operation::enable_key::builders::EnableKeyRequestBuilder {
        crate::operation::enable_key::builders::EnableKeyRequestBuilder::default()
    }
}

/// A builder for [`EnableKeyRequest`](crate::operation::operation::EnableKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnableKeyRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl EnableKeyRequestBuilder {
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
    /// Consumes the builder and constructs a [`EnableKeyRequest`](crate::operation::operation::EnableKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_key::EnableKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_key::EnableKeyRequest {
            key_id: self.key_id,
        })
    }
}
