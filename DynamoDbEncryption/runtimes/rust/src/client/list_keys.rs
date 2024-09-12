// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListKeys`](crate::operation::list_keys::builders::ListKeysFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_keys::builders::ListKeysFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_keys::builders::ListKeysFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_keys::builders::ListKeysFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_keys::builders::ListKeysFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListKeysResponse`](crate::operation::list_keys::ListKeysResponse) with field(s):
    ///   - [`keys(Option<::std::vec::Vec<kms::types::KeyListEntry>>)`](crate::operation::list_keys::ListKeysResponse::keys): (undocumented)
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_keys::ListKeysResponse::next_marker): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_keys::ListKeysResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListKeysError>`](crate::operation::list_keys::ListKeysError)
    pub fn list_keys(&self) -> crate::operation::list_keys::builders::ListKeysFluentBuilder {
        crate::operation::list_keys::builders::ListKeysFluentBuilder::new(self.clone())
    }
}
