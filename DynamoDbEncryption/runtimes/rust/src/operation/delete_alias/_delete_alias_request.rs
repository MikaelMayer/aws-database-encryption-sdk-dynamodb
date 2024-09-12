// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAliasRequest {
    #[allow(missing_docs)] // documentation missing in model
pub alias_name: ::std::option::Option<::std::string::String>,
}
impl DeleteAliasRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn alias_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.alias_name
}
}
impl DeleteAliasRequest {
    /// Creates a new builder-style object to manufacture [`DeleteAliasRequest`](crate::operation::delete_alias::builders::DeleteAliasRequest).
    pub fn builder() -> crate::operation::delete_alias::builders::DeleteAliasRequestBuilder {
        crate::operation::delete_alias::builders::DeleteAliasRequestBuilder::default()
    }
}

/// A builder for [`DeleteAliasRequest`](crate::operation::operation::DeleteAliasRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteAliasRequestBuilder {
    pub(crate) alias_name: ::std::option::Option<::std::string::String>,
}
impl DeleteAliasRequestBuilder {
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
    /// Consumes the builder and constructs a [`DeleteAliasRequest`](crate::operation::operation::DeleteAliasRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_alias::DeleteAliasRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_alias::DeleteAliasRequest {
            alias_name: self.alias_name,
        })
    }
}
