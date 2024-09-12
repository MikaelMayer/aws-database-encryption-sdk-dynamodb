// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScheduleKeyDeletionResponse {
    #[allow(missing_docs)] // documentation missing in model
pub deletion_date: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_state: ::std::option::Option<kms::types::KeyState>,
#[allow(missing_docs)] // documentation missing in model
pub pending_window_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl ScheduleKeyDeletionResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn deletion_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.deletion_date
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_state(&self) -> &::std::option::Option<kms::types::KeyState> {
    &self.key_state
}
#[allow(missing_docs)] // documentation missing in model
pub fn pending_window_in_days(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.pending_window_in_days
}
}
impl ScheduleKeyDeletionResponse {
    /// Creates a new builder-style object to manufacture [`ScheduleKeyDeletionResponse`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionResponse).
    pub fn builder() -> crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionResponseBuilder {
        crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionResponseBuilder::default()
    }
}

/// A builder for [`ScheduleKeyDeletionResponse`](crate::operation::operation::ScheduleKeyDeletionResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScheduleKeyDeletionResponseBuilder {
    pub(crate) deletion_date: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_state: ::std::option::Option<kms::types::KeyState>,
pub(crate) pending_window_in_days: ::std::option::Option<::std::primitive::i32>,
}
impl ScheduleKeyDeletionResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn deletion_date(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.deletion_date = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_deletion_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.deletion_date = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_deletion_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.deletion_date
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
#[allow(missing_docs)] // documentation missing in model
pub fn key_state(mut self, input: impl ::std::convert::Into<kms::types::KeyState>) -> Self {
    self.key_state = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_state(mut self, input: ::std::option::Option<kms::types::KeyState>) -> Self {
    self.key_state = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_state(&self) -> &::std::option::Option<kms::types::KeyState> {
    &self.key_state
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
    /// Consumes the builder and constructs a [`ScheduleKeyDeletionResponse`](crate::operation::operation::ScheduleKeyDeletionResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse {
            deletion_date: self.deletion_date,
key_id: self.key_id,
key_state: self.key_state,
pending_window_in_days: self.pending_window_in_days,
        })
    }
}
