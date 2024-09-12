// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListTagsOfResource`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<Option<::std::string::String>>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::next_token) / [`set_next_token(Option<::std::string::String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::set_next_token): (undocumented)<br>
    ///   - [`resource_arn(impl Into<Option<::std::string::String>>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<::std::string::String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::set_resource_arn): (undocumented)<br>
    /// - On success, responds with [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput) with field(s):
    ///   - [`next_token(Option<::std::string::String>)`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput::next_token): (undocumented)
    ///   - [`tags(Option<::std::vec::Vec<dynamodb::types::Tag>>)`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput::tags): (undocumented)
    /// - On failure, responds with [`SdkError<ListTagsOfResourceError>`](crate::operation::list_tags_of_resource::ListTagsOfResourceError)
    pub fn list_tags_of_resource(&self) -> crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder {
        crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::new(self.clone())
    }
}
