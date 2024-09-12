// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListKeyRotationsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub next_marker: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub rotations: ::std::option::Option<::std::vec::Vec<kms::types::RotationsListEntry>>,
#[allow(missing_docs)] // documentation missing in model
pub truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeyRotationsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_marker(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_marker
}
#[allow(missing_docs)] // documentation missing in model
pub fn rotations(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::RotationsListEntry>> {
    &self.rotations
}
#[allow(missing_docs)] // documentation missing in model
pub fn truncated(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.truncated
}
}
impl ListKeyRotationsResponse {
    /// Creates a new builder-style object to manufacture [`ListKeyRotationsResponse`](crate::operation::list_key_rotations::builders::ListKeyRotationsResponse).
    pub fn builder() -> crate::operation::list_key_rotations::builders::ListKeyRotationsResponseBuilder {
        crate::operation::list_key_rotations::builders::ListKeyRotationsResponseBuilder::default()
    }
}

/// A builder for [`ListKeyRotationsResponse`](crate::operation::operation::ListKeyRotationsResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListKeyRotationsResponseBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
pub(crate) rotations: ::std::option::Option<::std::vec::Vec<kms::types::RotationsListEntry>>,
pub(crate) truncated: ::std::option::Option<::std::primitive::bool>,
}
impl ListKeyRotationsResponseBuilder {
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
pub fn rotations(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::RotationsListEntry>>) -> Self {
    self.rotations = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_rotations(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::RotationsListEntry>>) -> Self {
    self.rotations = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_rotations(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::RotationsListEntry>> {
    &self.rotations
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
    /// Consumes the builder and constructs a [`ListKeyRotationsResponse`](crate::operation::operation::ListKeyRotationsResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_key_rotations::ListKeyRotationsResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_key_rotations::ListKeyRotationsResponse {
            next_marker: self.next_marker,
rotations: self.rotations,
truncated: self.truncated,
        })
    }
}
