// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RotateKeyOnDemandResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl RotateKeyOnDemandResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl RotateKeyOnDemandResponse {
    /// Creates a new builder-style object to manufacture [`RotateKeyOnDemandResponse`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandResponse).
    pub fn builder() -> crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandResponseBuilder {
        crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandResponseBuilder::default()
    }
}

/// A builder for [`RotateKeyOnDemandResponse`](crate::operation::operation::RotateKeyOnDemandResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RotateKeyOnDemandResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl RotateKeyOnDemandResponseBuilder {
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
    /// Consumes the builder and constructs a [`RotateKeyOnDemandResponse`](crate::operation::operation::RotateKeyOnDemandResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse {
            key_id: self.key_id,
        })
    }
}
