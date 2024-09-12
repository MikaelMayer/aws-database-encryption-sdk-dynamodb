// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCustomKeyStoresRequest {
    #[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub custom_key_store_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub marker: ::std::option::Option<::std::string::String>,
}
impl DescribeCustomKeyStoresRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.marker
}
}
impl DescribeCustomKeyStoresRequest {
    /// Creates a new builder-style object to manufacture [`DescribeCustomKeyStoresRequest`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresRequest).
    pub fn builder() -> crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresRequestBuilder {
        crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresRequestBuilder::default()
    }
}

/// A builder for [`DescribeCustomKeyStoresRequest`](crate::operation::operation::DescribeCustomKeyStoresRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeCustomKeyStoresRequestBuilder {
    pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
pub(crate) custom_key_store_name: ::std::option::Option<::std::string::String>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) marker: ::std::option::Option<::std::string::String>,
}
impl DescribeCustomKeyStoresRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.limit = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_limit(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.limit = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.marker = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.marker = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.marker
}
    /// Consumes the builder and constructs a [`DescribeCustomKeyStoresRequest`](crate::operation::operation::DescribeCustomKeyStoresRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresRequest {
            custom_key_store_id: self.custom_key_store_id,
custom_key_store_name: self.custom_key_store_name,
limit: self.limit,
marker: self.marker,
        })
    }
}
