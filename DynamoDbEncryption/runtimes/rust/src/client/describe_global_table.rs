// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeGlobalTable`](crate::operation::describe_global_table::builders::DescribeGlobalTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_global_table::builders::DescribeGlobalTableFluentBuilder::global_table_name) / [`set_global_table_name(Option<::std::string::String>)`](crate::operation::describe_global_table::builders::DescribeGlobalTableFluentBuilder::set_global_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeGlobalTableOutput`](crate::operation::describe_global_table::DescribeGlobalTableOutput) with field(s):
    ///   - [`global_table_description(Option<dynamodb::types::GlobalTableDescription>)`](crate::operation::describe_global_table::DescribeGlobalTableOutput::global_table_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeGlobalTableError>`](crate::operation::describe_global_table::DescribeGlobalTableError)
    pub fn describe_global_table(&self) -> crate::operation::describe_global_table::builders::DescribeGlobalTableFluentBuilder {
        crate::operation::describe_global_table::builders::DescribeGlobalTableFluentBuilder::new(self.clone())
    }
}
