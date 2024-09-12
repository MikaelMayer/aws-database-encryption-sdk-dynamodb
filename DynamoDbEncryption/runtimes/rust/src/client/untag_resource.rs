// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UntagResource`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<Option<::std::string::String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<::std::string::String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_resource_arn): (undocumented)<br>
    ///   - [`tag_keys(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::tag_keys) / [`set_tag_keys(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_tag_keys): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::untag_resource::Unit) with field(s):

    /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::operation::untag_resource::UntagResourceError)
    pub fn untag_resource(&self) -> crate::operation::untag_resource::builders::UntagResourceFluentBuilder {
        crate::operation::untag_resource::builders::UntagResourceFluentBuilder::new(self.clone())
    }
}
