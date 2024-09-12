// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourceTagsRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub marker: ::std::option::Option<::std::string::String>,
}
impl ListResourceTagsRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
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
impl ListResourceTagsRequest {
    /// Creates a new builder-style object to manufacture [`ListResourceTagsRequest`](crate::operation::list_resource_tags::builders::ListResourceTagsRequest).
    pub fn builder() -> crate::operation::list_resource_tags::builders::ListResourceTagsRequestBuilder {
        crate::operation::list_resource_tags::builders::ListResourceTagsRequestBuilder::default()
    }
}

/// A builder for [`ListResourceTagsRequest`](crate::operation::operation::ListResourceTagsRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResourceTagsRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) marker: ::std::option::Option<::std::string::String>,
}
impl ListResourceTagsRequestBuilder {
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
    /// Consumes the builder and constructs a [`ListResourceTagsRequest`](crate::operation::operation::ListResourceTagsRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_resource_tags::ListResourceTagsRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_resource_tags::ListResourceTagsRequest {
            key_id: self.key_id,
limit: self.limit,
marker: self.marker,
        })
    }
}
