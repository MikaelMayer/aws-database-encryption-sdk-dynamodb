// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScheduleKeyDeletionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub pending_window_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl ScheduleKeyDeletionRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn pending_window_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.pending_window_in_days
}
}
impl ScheduleKeyDeletionRequest {
    /// Creates a new builder-style object to manufacture [`ScheduleKeyDeletionRequest`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionRequest).
    pub fn builder() -> crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionRequestBuilder {
        crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionRequestBuilder::default()
    }
}

/// A builder for [`ScheduleKeyDeletionRequest`](crate::operation::operation::ScheduleKeyDeletionRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScheduleKeyDeletionRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) pending_window_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl ScheduleKeyDeletionRequestBuilder {
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
pub fn pending_window_in_days(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.pending_window_in_days = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_pending_window_in_days(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.pending_window_in_days = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_pending_window_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.pending_window_in_days
}
    /// Consumes the builder and constructs a [`ScheduleKeyDeletionRequest`](crate::operation::operation::ScheduleKeyDeletionRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest {
            key_id: self.key_id,
pending_window_in_days: self.pending_window_in_days,
        })
    }
}
