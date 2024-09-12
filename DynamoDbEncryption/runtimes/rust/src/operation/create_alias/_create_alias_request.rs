// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAliasRequest {
    #[allow(missing_docs)] // documentation missing in model
pub alias_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub target_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateAliasRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn alias_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.alias_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.target_key_id
}
}
impl CreateAliasRequest {
    /// Creates a new builder-style object to manufacture [`CreateAliasRequest`](crate::operation::create_alias::builders::CreateAliasRequest).
    pub fn builder() -> crate::operation::create_alias::builders::CreateAliasRequestBuilder {
        crate::operation::create_alias::builders::CreateAliasRequestBuilder::default()
    }
}

/// A builder for [`CreateAliasRequest`](crate::operation::operation::CreateAliasRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAliasRequestBuilder {
    pub(crate) alias_name: ::std::option::Option<::std::string::String>,
pub(crate) target_key_id: ::std::option::Option<::std::string::String>,
}
impl CreateAliasRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn alias_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.alias_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_alias_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.alias_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_alias_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.alias_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn target_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.target_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_target_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.target_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_target_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.target_key_id
}
    /// Consumes the builder and constructs a [`CreateAliasRequest`](crate::operation::operation::CreateAliasRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_alias::CreateAliasRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_alias::CreateAliasRequest {
            alias_name: self.alias_name,
target_key_id: self.target_key_id,
        })
    }
}
