// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCustomKeyStoresResponse {
    #[allow(missing_docs)] // documentation missing in model
pub custom_key_stores: ::std::option::Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>>,
#[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl DescribeCustomKeyStoresResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_stores(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>> {
    &self.custom_key_stores
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
impl DescribeCustomKeyStoresResponse {
    /// Creates a new builder-style object to manufacture [`DescribeCustomKeyStoresResponse`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresResponse).
    pub fn builder() -> crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresResponseBuilder {
        crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresResponseBuilder::default()
    }
}

/// A builder for [`DescribeCustomKeyStoresResponse`](crate::operation::operation::DescribeCustomKeyStoresResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeCustomKeyStoresResponseBuilder {
    pub(crate) custom_key_stores: ::std::option::Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>>,
pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl DescribeCustomKeyStoresResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_stores(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>>) -> Self {
    self.custom_key_stores = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_stores(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>>) -> Self {
    self.custom_key_stores = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_stores(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>> {
    &self.custom_key_stores
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
    /// Consumes the builder and constructs a [`DescribeCustomKeyStoresResponse`](crate::operation::operation::DescribeCustomKeyStoresResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse {
            custom_key_stores: self.custom_key_stores,
next_marker: self.next_marker,
truncated: self.truncated,
        })
    }
}
