// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAliasesResponse {
    #[allow(missing_docs)] // documentation missing in model
pub aliases: ::std::option::Option<::std::vec::Vec<kms::types::AliasListEntry>>,
#[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListAliasesResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn aliases(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::AliasListEntry>> {
    &self.aliases
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
impl ListAliasesResponse {
    /// Creates a new builder-style object to manufacture [`ListAliasesResponse`](crate::operation::list_aliases::builders::ListAliasesResponse).
    pub fn builder() -> crate::operation::list_aliases::builders::ListAliasesResponseBuilder {
        crate::operation::list_aliases::builders::ListAliasesResponseBuilder::default()
    }
}

/// A builder for [`ListAliasesResponse`](crate::operation::operation::ListAliasesResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAliasesResponseBuilder {
    pub(crate) aliases: ::std::option::Option<::std::vec::Vec<kms::types::AliasListEntry>>,
pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListAliasesResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn aliases(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::AliasListEntry>>) -> Self {
    self.aliases = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_aliases(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::AliasListEntry>>) -> Self {
    self.aliases = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_aliases(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::AliasListEntry>> {
    &self.aliases
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
    /// Consumes the builder and constructs a [`ListAliasesResponse`](crate::operation::operation::ListAliasesResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_aliases::ListAliasesResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_aliases::ListAliasesResponse {
            aliases: self.aliases,
next_marker: self.next_marker,
truncated: self.truncated,
        })
    }
}
