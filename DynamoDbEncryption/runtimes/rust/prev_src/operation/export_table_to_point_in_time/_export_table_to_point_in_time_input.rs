// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportTableToPointInTimeInput {
    #[allow(missing_docs)] // documentation missing in model
pub client_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub export_format: ::std::option::Option<dynamodb::types::ExportFormat>,
#[allow(missing_docs)] // documentation missing in model
pub export_time: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub s3_bucket: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub s3_bucket_owner: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub s3_prefix: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub s3_sse_algorithm: ::std::option::Option<dynamodb::types::S3SseAlgorithm>,
#[allow(missing_docs)] // documentation missing in model
pub s3_sse_kms_key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_arn: ::std::option::Option<::std::string::String>,
}
impl ExportTableToPointInTimeInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_format(&self) -> &::std::option::Option<dynamodb::types::ExportFormat> {
    &self.export_format
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.export_time
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_bucket
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_bucket_owner
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_prefix(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_prefix
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_algorithm(&self) -> &::std::option::Option<dynamodb::types::S3SseAlgorithm> {
    &self.s3_sse_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_sse_kms_key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_arn
}
}
impl ExportTableToPointInTimeInput {
    /// Creates a new builder-style object to manufacture [`ExportTableToPointInTimeInput`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeInput).
    pub fn builder() -> crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeInputBuilder {
        crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeInputBuilder::default()
    }
}

/// A builder for [`ExportTableToPointInTimeInput`](crate::operation::operation::ExportTableToPointInTimeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportTableToPointInTimeInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
pub(crate) export_format: ::std::option::Option<dynamodb::types::ExportFormat>,
pub(crate) export_time: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
pub(crate) s3_bucket_owner: ::std::option::Option<::std::string::String>,
pub(crate) s3_prefix: ::std::option::Option<::std::string::String>,
pub(crate) s3_sse_algorithm: ::std::option::Option<dynamodb::types::S3SseAlgorithm>,
pub(crate) s3_sse_kms_key_id: ::std::option::Option<::std::string::String>,
pub(crate) table_arn: ::std::option::Option<::std::string::String>,
}
impl ExportTableToPointInTimeInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.client_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.client_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_format(mut self, input: impl ::std::convert::Into<dynamodb::types::ExportFormat>) -> Self {
    self.export_format = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_format(mut self, input: ::std::option::Option<dynamodb::types::ExportFormat>) -> Self {
    self.export_format = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_format(&self) -> &::std::option::Option<dynamodb::types::ExportFormat> {
    &self.export_format
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_time(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.export_time = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.export_time = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.export_time
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.s3_bucket = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.s3_bucket = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_bucket
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.s3_bucket_owner = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.s3_bucket_owner = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_bucket_owner
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.s3_prefix = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.s3_prefix = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_prefix(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_prefix
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_algorithm(mut self, input: impl ::std::convert::Into<dynamodb::types::S3SseAlgorithm>) -> Self {
    self.s3_sse_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_sse_algorithm(mut self, input: ::std::option::Option<dynamodb::types::S3SseAlgorithm>) -> Self {
    self.s3_sse_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_sse_algorithm(&self) -> &::std::option::Option<dynamodb::types::S3SseAlgorithm> {
    &self.s3_sse_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.s3_sse_kms_key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_sse_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.s3_sse_kms_key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_sse_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.s3_sse_kms_key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_arn
}
    /// Consumes the builder and constructs a [`ExportTableToPointInTimeInput`](crate::operation::operation::ExportTableToPointInTimeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput {
            client_token: self.client_token,
export_format: self.export_format,
export_time: self.export_time,
s3_bucket: self.s3_bucket,
s3_bucket_owner: self.s3_bucket_owner,
s3_prefix: self.s3_prefix,
s3_sse_algorithm: self.s3_sse_algorithm,
s3_sse_kms_key_id: self.s3_sse_kms_key_id,
table_arn: self.table_arn,
        })
    }
}
