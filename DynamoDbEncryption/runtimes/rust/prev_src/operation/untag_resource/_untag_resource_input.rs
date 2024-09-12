// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UntagResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub resource_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn resource_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.resource_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn tag_keys(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.tag_keys
}
}
impl UntagResourceInput {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::operation::untag_resource::builders::UntagResourceInput).
    pub fn builder() -> crate::operation::untag_resource::builders::UntagResourceInputBuilder {
        crate::operation::untag_resource::builders::UntagResourceInputBuilder::default()
    }
}

/// A builder for [`UntagResourceInput`](crate::operation::operation::UntagResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UntagResourceInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
pub(crate) tag_keys: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl UntagResourceInputBuilder {
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
    /// Consumes the builder and constructs a [`UntagResourceInput`](crate::operation::operation::UntagResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::untag_resource::UntagResourceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::untag_resource::UntagResourceInput {
            resource_arn: self.resource_arn,
tag_keys: self.tag_keys,
        })
    }
}
