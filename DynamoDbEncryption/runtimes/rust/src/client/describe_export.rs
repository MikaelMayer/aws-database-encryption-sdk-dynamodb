// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeExport`](crate::operation::describe_export::builders::DescribeExportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_arn(impl Into<Option<::std::string::String>>)`](crate::operation::describe_export::builders::DescribeExportFluentBuilder::export_arn) / [`set_export_arn(Option<::std::string::String>)`](crate::operation::describe_export::builders::DescribeExportFluentBuilder::set_export_arn): (undocumented)<br>
    /// - On success, responds with [`DescribeExportOutput`](crate::operation::describe_export::DescribeExportOutput) with field(s):
    ///   - [`export_description(Option<dynamodb::types::ExportDescription>)`](crate::operation::describe_export::DescribeExportOutput::export_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeExportError>`](crate::operation::describe_export::DescribeExportError)
    pub fn describe_export(&self) -> crate::operation::describe_export::builders::DescribeExportFluentBuilder {
        crate::operation::describe_export::builders::DescribeExportFluentBuilder::new(self.clone())
    }
}
