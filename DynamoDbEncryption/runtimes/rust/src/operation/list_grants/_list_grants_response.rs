// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListGrantsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub grants: ::std::option::Option<::std::vec::Vec<kms::types::GrantListEntry>>,
#[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListGrantsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn grants(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::GrantListEntry>> {
    &self.grants
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
}
impl ListGrantsResponse {
    /// Creates a new builder-style object to manufacture [`ListGrantsResponse`](crate::operation::list_grants::builders::ListGrantsResponse).
    pub fn builder() -> crate::operation::list_grants::builders::ListGrantsResponseBuilder {
        crate::operation::list_grants::builders::ListGrantsResponseBuilder::default()
    }
}

/// A builder for [`ListGrantsResponse`](crate::operation::operation::ListGrantsResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListGrantsResponseBuilder {
    pub(crate) grants: ::std::option::Option<::std::vec::Vec<kms::types::GrantListEntry>>,
pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListGrantsResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn grants(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::GrantListEntry>>) -> Self {
    self.grants = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grants(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::GrantListEntry>>) -> Self {
    self.grants = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grants(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::GrantListEntry>> {
    &self.grants
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.next_marker = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.next_marker = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.truncated = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_truncated(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.truncated = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
    /// Consumes the builder and constructs a [`ListGrantsResponse`](crate::operation::operation::ListGrantsResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_grants::ListGrantsResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_grants::ListGrantsResponse {
            grants: self.grants,
next_marker: self.next_marker,
truncated: self.truncated,
        })
    }
}
