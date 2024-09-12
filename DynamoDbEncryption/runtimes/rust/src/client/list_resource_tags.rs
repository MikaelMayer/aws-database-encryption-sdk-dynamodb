// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListResourceTags`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListResourceTagsResponse`](crate::operation::list_resource_tags::ListResourceTagsResponse) with field(s):
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_resource_tags::ListResourceTagsResponse::next_marker): (undocumented)
    ///   - [`tags(Option<::std::vec::Vec<kms::types::Tag>>)`](crate::operation::list_resource_tags::ListResourceTagsResponse::tags): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_resource_tags::ListResourceTagsResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListResourceTagsError>`](crate::operation::list_resource_tags::ListResourceTagsError)
    pub fn list_resource_tags(&self) -> crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder {
        crate::operation::list_resource_tags::builders::ListResourceTagsFluentBuilder::new(self.clone())
    }
}
