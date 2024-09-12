// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeImport`](crate::operation::describe_import::builders::DescribeImportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`import_arn(impl Into<Option<::std::string::String>>)`](crate::operation::describe_import::builders::DescribeImportFluentBuilder::import_arn) / [`set_import_arn(Option<::std::string::String>)`](crate::operation::describe_import::builders::DescribeImportFluentBuilder::set_import_arn): (undocumented)<br>
    /// - On success, responds with [`DescribeImportOutput`](crate::operation::describe_import::DescribeImportOutput) with field(s):
    ///   - [`import_table_description(Option<dynamodb::types::ImportTableDescription>)`](crate::operation::describe_import::DescribeImportOutput::import_table_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeImportError>`](crate::operation::describe_import::DescribeImportError)
    pub fn describe_import(&self) -> crate::operation::describe_import::builders::DescribeImportFluentBuilder {
        crate::operation::describe_import::builders::DescribeImportFluentBuilder::new(self.clone())
    }
}
