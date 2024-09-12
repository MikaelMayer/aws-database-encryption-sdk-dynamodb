// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeTable`](crate::operation::describe_table::builders::DescribeTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_table::builders::DescribeTableFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_table::builders::DescribeTableFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeTableOutput`](crate::operation::describe_table::DescribeTableOutput) with field(s):
    ///   - [`table(Option<dynamodb::types::TableDescription>)`](crate::operation::describe_table::DescribeTableOutput::table): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeTableError>`](crate::operation::describe_table::DescribeTableError)
    pub fn describe_table(&self) -> crate::operation::describe_table::builders::DescribeTableFluentBuilder {
        crate::operation::describe_table::builders::DescribeTableFluentBuilder::new(self.clone())
    }
}
