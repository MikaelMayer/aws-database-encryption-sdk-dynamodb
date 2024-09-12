// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`EnableKinesisStreamingDestination`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_arn(impl Into<Option<::std::string::String>>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<::std::string::String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_stream_arn): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput) with field(s):
    ///   - [`destination_status(Option<dynamodb::types::DestinationStatus>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::destination_status): (undocumented)
    ///   - [`stream_arn(Option<::std::string::String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::stream_arn): (undocumented)
    ///   - [`table_name(Option<::std::string::String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::table_name): (undocumented)
    /// - On failure, responds with [`SdkError<EnableKinesisStreamingDestinationError>`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError)
    pub fn enable_kinesis_streaming_destination(&self) -> crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder {
        crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::new(self.clone())
    }
}
