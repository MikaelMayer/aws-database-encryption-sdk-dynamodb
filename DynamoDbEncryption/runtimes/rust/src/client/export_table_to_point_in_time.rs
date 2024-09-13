// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ExportTableToPointInTime`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::client_token) / [`set_client_token(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_client_token): (undocumented)<br>
    ///   - [`export_format(impl Into<Option<dynamodb::types::ExportFormat>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::export_format) / [`set_export_format(Option<dynamodb::types::ExportFormat>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_export_format): (undocumented)<br>
    ///   - [`export_time(impl Into<Option<::aws_smithy_types::DateTime>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::export_time) / [`set_export_time(Option<::aws_smithy_types::DateTime>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_export_time): (undocumented)<br>
    ///   - [`s3_bucket(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_bucket) / [`set_s3_bucket(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_bucket): (undocumented)<br>
    ///   - [`s3_bucket_owner(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_bucket_owner) / [`set_s3_bucket_owner(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_bucket_owner): (undocumented)<br>
    ///   - [`s3_prefix(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_prefix) / [`set_s3_prefix(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_prefix): (undocumented)<br>
    ///   - [`s3_sse_algorithm(impl Into<Option<dynamodb::types::S3SseAlgorithm>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_sse_algorithm) / [`set_s3_sse_algorithm(Option<dynamodb::types::S3SseAlgorithm>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_sse_algorithm): (undocumented)<br>
    ///   - [`s3_sse_kms_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::s3_sse_kms_key_id) / [`set_s3_sse_kms_key_id(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_s3_sse_kms_key_id): (undocumented)<br>
    ///   - [`table_arn(impl Into<Option<::std::string::String>>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::table_arn) / [`set_table_arn(Option<::std::string::String>)`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::set_table_arn): (undocumented)<br>
    /// - On success, responds with [`ExportTableToPointInTimeOutput`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput) with field(s):
    ///   - [`export_description(Option<dynamodb::types::ExportDescription>)`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput::export_description): (undocumented)
    /// - On failure, responds with [`SdkError<ExportTableToPointInTimeError>`](crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeError)
    pub fn export_table_to_point_in_time(&self) -> crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder {
        crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeFluentBuilder::new(self.clone())
    }
}