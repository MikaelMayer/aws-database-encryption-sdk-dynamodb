// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub resource_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl TagResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn resource_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.resource_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    &self.tags
}
}
impl TagResourceInput {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::operation::tag_resource::builders::TagResourceInput).
    pub fn builder() -> crate::operation::tag_resource::builders::TagResourceInputBuilder {
        crate::operation::tag_resource::builders::TagResourceInputBuilder::default()
    }
}

/// A builder for [`TagResourceInput`](crate::operation::operation::TagResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl TagResourceInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.resource_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.resource_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.resource_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.tags = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>) -> Self {
    self.tags = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    &self.tags
}
    /// Consumes the builder and constructs a [`TagResourceInput`](crate::operation::operation::TagResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_resource::TagResourceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::tag_resource::TagResourceInput {
            resource_arn: self.resource_arn,
tags: self.tags,
        })
    }
}