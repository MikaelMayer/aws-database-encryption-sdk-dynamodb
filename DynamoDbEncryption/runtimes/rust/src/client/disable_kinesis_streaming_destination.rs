// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DisableKinesisStreamingDestination`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_arn(impl Into<Option<::std::string::String>>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<::std::string::String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::set_stream_arn): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DisableKinesisStreamingDestinationOutput`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput) with field(s):
    ///   - [`destination_status(Option<dynamodb::types::DestinationStatus>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::destination_status): (undocumented)
    ///   - [`stream_arn(Option<::std::string::String>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::stream_arn): (undocumented)
    ///   - [`table_name(Option<::std::string::String>)`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput::table_name): (undocumented)
    /// - On failure, responds with [`SdkError<DisableKinesisStreamingDestinationError>`](crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationError)
    pub fn disable_kinesis_streaming_destination(&self) -> crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder {
        crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationFluentBuilder::new(self.clone())
    }
}
