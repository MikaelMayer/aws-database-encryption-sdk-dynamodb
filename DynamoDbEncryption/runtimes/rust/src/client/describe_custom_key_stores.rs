// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeCustomKeyStores`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    ///   - [`custom_key_store_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::custom_key_store_name) / [`set_custom_key_store_name(Option<::std::string::String>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::set_custom_key_store_name): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`DescribeCustomKeyStoresResponse`](crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse) with field(s):
    ///   - [`custom_key_stores(Option<::std::vec::Vec<kms::types::CustomKeyStoresListEntry>>)`](crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse::custom_key_stores): (undocumented)
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse::next_marker): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeCustomKeyStoresError>`](crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError)
    pub fn describe_custom_key_stores(&self) -> crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder {
        crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresFluentBuilder::new(self.clone())
    }
}
