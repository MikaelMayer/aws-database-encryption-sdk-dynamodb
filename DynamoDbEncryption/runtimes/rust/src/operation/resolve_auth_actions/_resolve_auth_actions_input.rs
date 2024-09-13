// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResolveAuthActionsInput {
    #[allow(missing_docs)] // documentation missing in model
pub auth_actions: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>,
#[allow(missing_docs)] // documentation missing in model
pub header_bytes: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl ResolveAuthActionsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn auth_actions(&self) -> &::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>> {
    &self.auth_actions
}
#[allow(missing_docs)] // documentation missing in model
pub fn header_bytes(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.header_bytes
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl ResolveAuthActionsInput {
    /// Creates a new builder-style object to manufacture [`ResolveAuthActionsInput`](crate::operation::resolve_auth_actions::builders::ResolveAuthActionsInput).
    pub fn builder() -> crate::operation::resolve_auth_actions::builders::ResolveAuthActionsInputBuilder {
        crate::operation::resolve_auth_actions::builders::ResolveAuthActionsInputBuilder::default()
    }
}

/// A builder for [`ResolveAuthActionsInput`](crate::operation::operation::ResolveAuthActionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResolveAuthActionsInputBuilder {
    pub(crate) auth_actions: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>,
pub(crate) header_bytes: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl ResolveAuthActionsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn auth_actions(mut self, input: impl ::std::convert::Into<::std::vec::Vec<structured_encryption::types::AuthItem>>) -> Self {
    self.auth_actions = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_auth_actions(mut self, input: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>) -> Self {
    self.auth_actions = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_auth_actions(&self) -> &::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>> {
    &self.auth_actions
}
#[allow(missing_docs)] // documentation missing in model
pub fn header_bytes(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.header_bytes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_header_bytes(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.header_bytes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_header_bytes(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.header_bytes
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`ResolveAuthActionsInput`](crate::operation::operation::ResolveAuthActionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::resolve_auth_actions::ResolveAuthActionsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::resolve_auth_actions::ResolveAuthActionsInput {
            auth_actions: self.auth_actions,
header_bytes: self.header_bytes,
table_name: self.table_name,
        })
    }
}