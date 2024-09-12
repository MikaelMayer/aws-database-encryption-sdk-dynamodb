// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UntagResourceRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.tag_keys
}
}
impl UntagResourceRequest {
    /// Creates a new builder-style object to manufacture [`UntagResourceRequest`](crate::operation::untag_resource::builders::UntagResourceRequest).
    pub fn builder() -> crate::operation::untag_resource::builders::UntagResourceRequestBuilder {
        crate::operation::untag_resource::builders::UntagResourceRequestBuilder::default()
    }
}

/// A builder for [`UntagResourceRequest`](crate::operation::operation::UntagResourceRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UntagResourceRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceRequestBuilder {
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
pub fn tag_keys(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.tag_keys = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tag_keys(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.tag_keys = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.tag_keys
}
    /// Consumes the builder and constructs a [`UntagResourceRequest`](crate::operation::operation::UntagResourceRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::untag_resource::UntagResourceRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::untag_resource::UntagResourceRequest {
            key_id: self.key_id,
tag_keys: self.tag_keys,
        })
    }
}
