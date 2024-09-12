// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelKeyDeletionResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl CancelKeyDeletionResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl CancelKeyDeletionResponse {
    /// Creates a new builder-style object to manufacture [`CancelKeyDeletionResponse`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionResponse).
    pub fn builder() -> crate::operation::cancel_key_deletion::builders::CancelKeyDeletionResponseBuilder {
        crate::operation::cancel_key_deletion::builders::CancelKeyDeletionResponseBuilder::default()
    }
}

/// A builder for [`CancelKeyDeletionResponse`](crate::operation::operation::CancelKeyDeletionResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelKeyDeletionResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl CancelKeyDeletionResponseBuilder {
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
    /// Consumes the builder and constructs a [`CancelKeyDeletionResponse`](crate::operation::operation::CancelKeyDeletionResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_key_deletion::CancelKeyDeletionResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::cancel_key_deletion::CancelKeyDeletionResponse {
            key_id: self.key_id,
        })
    }
}
