// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPublicKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl GetPublicKeyRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.grant_tokens
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl GetPublicKeyRequest {
    /// Creates a new builder-style object to manufacture [`GetPublicKeyRequest`](crate::operation::get_public_key::builders::GetPublicKeyRequest).
    pub fn builder() -> crate::operation::get_public_key::builders::GetPublicKeyRequestBuilder {
        crate::operation::get_public_key::builders::GetPublicKeyRequestBuilder::default()
    }
}

/// A builder for [`GetPublicKeyRequest`](crate::operation::operation::GetPublicKeyRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPublicKeyRequestBuilder {
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl GetPublicKeyRequestBuilder {
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
    /// Consumes the builder and constructs a [`GetPublicKeyRequest`](crate::operation::operation::GetPublicKeyRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_public_key::GetPublicKeyRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_public_key::GetPublicKeyRequest {
            grant_tokens: self.grant_tokens,
key_id: self.key_id,
        })
    }
}
