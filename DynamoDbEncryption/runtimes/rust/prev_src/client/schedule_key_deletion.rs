// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ScheduleKeyDeletion`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`pending_window_in_days(impl Into<Option<::std::primitive::i32>>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::pending_window_in_days) / [`set_pending_window_in_days(Option<::std::primitive::i32>)`](crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::set_pending_window_in_days): (undocumented)<br>
    /// - On success, responds with [`ScheduleKeyDeletionResponse`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse) with field(s):
    ///   - [`deletion_date(Option<::aws_smithy_types::DateTime>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse::deletion_date): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse::key_id): (undocumented)
    ///   - [`key_state(Option<kms::types::KeyState>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse::key_state): (undocumented)
    ///   - [`pending_window_in_days(Option<::std::primitive::i32>)`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionResponse::pending_window_in_days): (undocumented)
    /// - On failure, responds with [`SdkError<ScheduleKeyDeletionError>`](crate::operation::schedule_key_deletion::ScheduleKeyDeletionError)
    pub fn schedule_key_deletion(&self) -> crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder {
        crate::operation::schedule_key_deletion::builders::ScheduleKeyDeletionFluentBuilder::new(self.clone())
    }
}
