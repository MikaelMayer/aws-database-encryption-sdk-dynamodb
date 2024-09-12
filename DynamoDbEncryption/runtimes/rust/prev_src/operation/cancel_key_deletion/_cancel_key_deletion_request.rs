// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelKeyDeletionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl CancelKeyDeletionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl CancelKeyDeletionRequest {
    /// Creates a new builder-style object to manufacture [`CancelKeyDeletionRequest`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionRequest).
    pub fn builder() -> crate::operation::cancel_key_deletion::builders::CancelKeyDeletionRequestBuilder {
        crate::operation::cancel_key_deletion::builders::CancelKeyDeletionRequestBuilder::default()
    }
}

/// A builder for [`CancelKeyDeletionRequest`](crate::operation::operation::CancelKeyDeletionRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelKeyDeletionRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl CancelKeyDeletionRequestBuilder {
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
    /// Consumes the builder and constructs a [`CancelKeyDeletionRequest`](crate::operation::operation::CancelKeyDeletionRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_key_deletion::CancelKeyDeletionRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::cancel_key_deletion::CancelKeyDeletionRequest {
            key_id: self.key_id,
        })
    }
}
