// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagResourceRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl TagResourceRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::Tag>> {
    &self.tags
}
}
impl TagResourceRequest {
    /// Creates a new builder-style object to manufacture [`TagResourceRequest`](crate::operation::tag_resource::builders::TagResourceRequest).
    pub fn builder() -> crate::operation::tag_resource::builders::TagResourceRequestBuilder {
        crate::operation::tag_resource::builders::TagResourceRequestBuilder::default()
    }
}

/// A builder for [`TagResourceRequest`](crate::operation::operation::TagResourceRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagResourceRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<kms::types::Tag>>,
}
impl TagResourceRequestBuilder {
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
    /// Consumes the builder and constructs a [`TagResourceRequest`](crate::operation::operation::TagResourceRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_resource::TagResourceRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::tag_resource::TagResourceRequest {
            key_id: self.key_id,
tags: self.tags,
        })
    }
}
