// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeKinesisStreamingDestination`](crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeKinesisStreamingDestinationOutput`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput) with field(s):
    ///   - [`kinesis_data_stream_destinations(Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>>)`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput::kinesis_data_stream_destinations): (undocumented)
    ///   - [`table_name(Option<::std::string::String>)`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput::table_name): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeKinesisStreamingDestinationError>`](crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError)
    pub fn describe_kinesis_streaming_destination(&self) -> crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder {
        crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationFluentBuilder::new(self.clone())
    }
}
