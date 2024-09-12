// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsOfResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl ListTagsOfResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn resource_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.resource_arn
}
}
impl ListTagsOfResourceInput {
    /// Creates a new builder-style object to manufacture [`ListTagsOfResourceInput`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceInput).
    pub fn builder() -> crate::operation::list_tags_of_resource::builders::ListTagsOfResourceInputBuilder {
        crate::operation::list_tags_of_resource::builders::ListTagsOfResourceInputBuilder::default()
    }
}

/// A builder for [`ListTagsOfResourceInput`](crate::operation::operation::ListTagsOfResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTagsOfResourceInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl ListTagsOfResourceInputBuilder {
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
    /// Consumes the builder and constructs a [`ListTagsOfResourceInput`](crate::operation::operation::ListTagsOfResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tags_of_resource::ListTagsOfResourceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_tags_of_resource::ListTagsOfResourceInput {
            next_token: self.next_token,
resource_arn: self.resource_arn,
        })
    }
}
