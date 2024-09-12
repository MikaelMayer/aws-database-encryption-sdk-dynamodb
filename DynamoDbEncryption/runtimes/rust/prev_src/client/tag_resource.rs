// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`tags(impl Into<Option<::std::vec::Vec<kms::types::Tag>>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<::std::vec::Vec<kms::types::Tag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::tag_resource::Unit) with field(s):

    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(&self) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.clone())
    }
}
