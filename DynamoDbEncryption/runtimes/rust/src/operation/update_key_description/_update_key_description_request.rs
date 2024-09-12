// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateKeyDescriptionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub description: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl UpdateKeyDescriptionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn description(&self) -> &::std::option::Option<::std::string::String> {
    &self.description
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl UpdateKeyDescriptionRequest {
    /// Creates a new builder-style object to manufacture [`UpdateKeyDescriptionRequest`](crate::operation::update_key_description::builders::UpdateKeyDescriptionRequest).
    pub fn builder() -> crate::operation::update_key_description::builders::UpdateKeyDescriptionRequestBuilder {
        crate::operation::update_key_description::builders::UpdateKeyDescriptionRequestBuilder::default()
    }
}

/// A builder for [`UpdateKeyDescriptionRequest`](crate::operation::operation::UpdateKeyDescriptionRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateKeyDescriptionRequestBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl UpdateKeyDescriptionRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
    &self.description
}
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
    /// Consumes the builder and constructs a [`UpdateKeyDescriptionRequest`](crate::operation::operation::UpdateKeyDescriptionRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_key_description::UpdateKeyDescriptionRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_key_description::UpdateKeyDescriptionRequest {
            description: self.description,
key_id: self.key_id,
        })
    }
}
