// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeKey`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::describe_key::builders::DescribeKeyFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`DescribeKeyResponse`](crate::operation::describe_key::DescribeKeyResponse) with field(s):
    ///   - [`key_metadata(Option<kms::types::KeyMetadata>)`](crate::operation::describe_key::DescribeKeyResponse::key_metadata): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeKeyError>`](crate::operation::describe_key::DescribeKeyError)
    pub fn describe_key(&self) -> crate::operation::describe_key::builders::DescribeKeyFluentBuilder {
        crate::operation::describe_key::builders::DescribeKeyFluentBuilder::new(self.clone())
    }
}
