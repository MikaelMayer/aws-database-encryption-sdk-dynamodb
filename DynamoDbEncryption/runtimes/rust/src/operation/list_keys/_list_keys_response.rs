// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListKeysResponse {
    #[allow(missing_docs)] // documentation missing in model
pub keys: ::std::option::Option<::std::vec::Vec<kms::types::KeyListEntry>>,
#[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeysResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn keys(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::KeyListEntry>> {
    &self.keys
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
impl ListKeysResponse {
    /// Creates a new builder-style object to manufacture [`ListKeysResponse`](crate::operation::list_keys::builders::ListKeysResponse).
    pub fn builder() -> crate::operation::list_keys::builders::ListKeysResponseBuilder {
        crate::operation::list_keys::builders::ListKeysResponseBuilder::default()
    }
}

/// A builder for [`ListKeysResponse`](crate::operation::operation::ListKeysResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListKeysResponseBuilder {
    pub(crate) keys: ::std::option::Option<::std::vec::Vec<kms::types::KeyListEntry>>,
pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeysResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn keys(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::KeyListEntry>>) -> Self {
    self.keys = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_keys(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::KeyListEntry>>) -> Self {
    self.keys = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_keys(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::KeyListEntry>> {
    &self.keys
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
    /// Consumes the builder and constructs a [`ListKeysResponse`](crate::operation::operation::ListKeysResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_keys::ListKeysResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_keys::ListKeysResponse {
            keys: self.keys,
next_marker: self.next_marker,
truncated: self.truncated,
        })
    }
}
