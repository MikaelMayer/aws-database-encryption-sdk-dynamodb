// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdatePrimaryRegionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub primary_region: ::std::option::Option<::std::string::String>,
}
impl UpdatePrimaryRegionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn primary_region(&self) -> &::std::option::Option<::std::string::String> {
    &self.primary_region
}
}
impl UpdatePrimaryRegionRequest {
    /// Creates a new builder-style object to manufacture [`UpdatePrimaryRegionRequest`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionRequest).
    pub fn builder() -> crate::operation::update_primary_region::builders::UpdatePrimaryRegionRequestBuilder {
        crate::operation::update_primary_region::builders::UpdatePrimaryRegionRequestBuilder::default()
    }
}

/// A builder for [`UpdatePrimaryRegionRequest`](crate::operation::operation::UpdatePrimaryRegionRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdatePrimaryRegionRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) primary_region: ::std::option::Option<::std::string::String>,
}
impl UpdatePrimaryRegionRequestBuilder {
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
pub fn primary_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.primary_region = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_primary_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.primary_region = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_primary_region(&self) -> &::std::option::Option<::std::string::String> {
    &self.primary_region
}
    /// Consumes the builder and constructs a [`UpdatePrimaryRegionRequest`](crate::operation::operation::UpdatePrimaryRegionRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_primary_region::UpdatePrimaryRegionRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_primary_region::UpdatePrimaryRegionRequest {
            key_id: self.key_id,
primary_region: self.primary_region,
        })
    }
}
