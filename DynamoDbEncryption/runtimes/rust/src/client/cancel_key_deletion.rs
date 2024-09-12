// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CancelKeyDeletion`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`CancelKeyDeletionResponse`](crate::operation::cancel_key_deletion::CancelKeyDeletionResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::cancel_key_deletion::CancelKeyDeletionResponse::key_id): (undocumented)
    /// - On failure, responds with [`SdkError<CancelKeyDeletionError>`](crate::operation::cancel_key_deletion::CancelKeyDeletionError)
    pub fn cancel_key_deletion(&self) -> crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder {
        crate::operation::cancel_key_deletion::builders::CancelKeyDeletionFluentBuilder::new(self.clone())
    }
}
