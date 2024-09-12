// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RetireGrantRequest {
    #[allow(missing_docs)] // documentation missing in model
pub dry_run: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub grant_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub grant_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl RetireGrantRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl RetireGrantRequest {
    /// Creates a new builder-style object to manufacture [`RetireGrantRequest`](crate::operation::retire_grant::builders::RetireGrantRequest).
    pub fn builder() -> crate::operation::retire_grant::builders::RetireGrantRequestBuilder {
        crate::operation::retire_grant::builders::RetireGrantRequestBuilder::default()
    }
}

/// A builder for [`RetireGrantRequest`](crate::operation::operation::RetireGrantRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RetireGrantRequestBuilder {
    pub(crate) dry_run: ::std::option::Option<::std::primitive::bool>,
pub(crate) grant_id: ::std::option::Option<::std::string::String>,
pub(crate) grant_token: ::std::option::Option<::std::string::String>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl RetireGrantRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn dry_run(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.dry_run = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_dry_run(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.dry_run = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.dry_run
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.grant_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.grant_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.grant_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.grant_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_token
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
    /// Consumes the builder and constructs a [`RetireGrantRequest`](crate::operation::operation::RetireGrantRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::retire_grant::RetireGrantRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::retire_grant::RetireGrantRequest {
            dry_run: self.dry_run,
grant_id: self.grant_id,
grant_token: self.grant_token,
key_id: self.key_id,
        })
    }
}
