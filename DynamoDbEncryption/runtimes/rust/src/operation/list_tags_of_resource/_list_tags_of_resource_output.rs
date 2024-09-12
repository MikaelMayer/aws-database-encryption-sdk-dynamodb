// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsOfResourceOutput {
    #[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl ListTagsOfResourceOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn tags(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>> {
    &self.tags
}
}
impl ListTagsOfResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceOutput).
    pub fn builder() -> crate::operation::list_tags_of_resource::builders::ListTagsOfResourceOutputBuilder {
        crate::operation::list_tags_of_resource::builders::ListTagsOfResourceOutputBuilder::default()
    }
}

/// A builder for [`ListTagsOfResourceOutput`](crate::operation::operation::ListTagsOfResourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTagsOfResourceOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) tags: ::std::option::Option<::std::vec::Vec<dynamodb::types::Tag>>,
}
impl ListTagsOfResourceOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.next_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.next_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
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
    /// Consumes the builder and constructs a [`ListTagsOfResourceOutput`](crate::operation::operation::ListTagsOfResourceOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tags_of_resource::ListTagsOfResourceOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_tags_of_resource::ListTagsOfResourceOutput {
            next_token: self.next_token,
tags: self.tags,
        })
    }
}
