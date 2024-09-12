// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::export_table_to_point_in_time::_export_table_to_point_in_time_output::ExportTableToPointInTimeOutputBuilder;

pub use crate::operation::export_table_to_point_in_time::_export_table_to_point_in_time_input::ExportTableToPointInTimeInputBuilder;

impl ExportTableToPointInTimeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.export_table_to_point_in_time();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExportTableToPointInTime`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExportTableToPointInTimeFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeInputBuilder,
}
impl ExportTableToPointInTimeFluentBuilder {
    /// Creates a new `ExportTableToPointInTime`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ExportTableToPointInTime as a reference.
    pub fn as_input(&self) -> &crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTime::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.client_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_client_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_client_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_format(mut self, input: impl ::std::convert::Into<dynamodb::types::ExportFormat>) -> Self {
    self.inner = self.inner.export_format(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_format(mut self, input: ::std::option::Option<dynamodb::types::ExportFormat>) -> Self {
    self.inner = self.inner.set_export_format(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_format(&self) -> &::std::option::Option<dynamodb::types::ExportFormat> {
    self.inner.get_export_format()
}
#[allow(missing_docs)] // documentation missing in model
pub fn export_time(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.export_time(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.set_export_time(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    self.inner.get_export_time()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.s3_bucket(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_s3_bucket(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_s3_bucket()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.s3_bucket_owner(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_s3_bucket_owner(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_s3_bucket_owner()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.s3_prefix(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_s3_prefix(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_prefix(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_s3_prefix()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_algorithm(mut self, input: impl ::std::convert::Into<dynamodb::types::S3SseAlgorithm>) -> Self {
    self.inner = self.inner.s3_sse_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_sse_algorithm(mut self, input: ::std::option::Option<dynamodb::types::S3SseAlgorithm>) -> Self {
    self.inner = self.inner.set_s3_sse_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_sse_algorithm(&self) -> &::std::option::Option<dynamodb::types::S3SseAlgorithm> {
    self.inner.get_s3_sse_algorithm()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_sse_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.s3_sse_kms_key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_sse_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_s3_sse_kms_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_sse_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_s3_sse_kms_key_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_arn(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_arn(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_arn()
}
}
