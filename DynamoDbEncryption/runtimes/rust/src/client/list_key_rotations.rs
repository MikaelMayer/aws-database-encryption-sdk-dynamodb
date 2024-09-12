// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListKeyRotations`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListKeyRotationsResponse`](crate::operation::list_key_rotations::ListKeyRotationsResponse) with field(s):
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_key_rotations::ListKeyRotationsResponse::next_marker): (undocumented)
    ///   - [`rotations(Option<::std::vec::Vec<kms::types::RotationsListEntry>>)`](crate::operation::list_key_rotations::ListKeyRotationsResponse::rotations): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_key_rotations::ListKeyRotationsResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListKeyRotationsError>`](crate::operation::list_key_rotations::ListKeyRotationsError)
    pub fn list_key_rotations(&self) -> crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder {
        crate::operation::list_key_rotations::builders::ListKeyRotationsFluentBuilder::new(self.clone())
    }
}
