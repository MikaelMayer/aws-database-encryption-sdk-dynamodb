// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourceTagsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListResourceTagsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
}
impl ListResourceTagsResponse {
    /// Creates a new builder-style object to manufacture [`ListResourceTagsResponse`](crate::operation::list_resource_tags::builders::ListResourceTagsResponse).
    pub fn builder() -> crate::operation::list_resource_tags::builders::ListResourceTagsResponseBuilder {
        crate::operation::list_resource_tags::builders::ListResourceTagsResponseBuilder::default()
    }
}

/// A builder for [`ListResourceTagsResponse`](crate::operation::operation::ListResourceTagsResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResourceTagsResponseBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListResourceTagsResponseBuilder {
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
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.tags = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>) -> Self {
    self.tags = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
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
    /// Consumes the builder and constructs a [`ListResourceTagsResponse`](crate::operation::operation::ListResourceTagsResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_resource_tags::ListResourceTagsResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_resource_tags::ListResourceTagsResponse {
            next_marker: self.next_marker,
tags: self.tags,
truncated: self.truncated,
        })
    }
}
