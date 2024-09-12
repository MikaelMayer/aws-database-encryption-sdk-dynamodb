// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGrantResponse {
    #[allow(missing_docs)] // documentation missing in model
pub grant_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub grant_token: ::std::option::Option<::std::string::String>,
}
impl CreateGrantResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn grant_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.grant_token
}
}
impl CreateGrantResponse {
    /// Creates a new builder-style object to manufacture [`CreateGrantResponse`](crate::operation::create_grant::builders::CreateGrantResponse).
    pub fn builder() -> crate::operation::create_grant::builders::CreateGrantResponseBuilder {
        crate::operation::create_grant::builders::CreateGrantResponseBuilder::default()
    }
}

/// A builder for [`CreateGrantResponse`](crate::operation::operation::CreateGrantResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateGrantResponseBuilder {
    pub(crate) grant_id: ::std::option::Option<::std::string::String>,
pub(crate) grant_token: ::std::option::Option<::std::string::String>,
}
impl CreateGrantResponseBuilder {
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
    /// Consumes the builder and constructs a [`CreateGrantResponse`](crate::operation::operation::CreateGrantResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_grant::CreateGrantResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_grant::CreateGrantResponse {
            grant_id: self.grant_id,
grant_token: self.grant_token,
        })
    }
}
