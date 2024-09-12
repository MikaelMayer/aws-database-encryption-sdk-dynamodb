// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeKinesisStreamingDestinationOutput {
    #[allow(missing_docs)] // documentation missing in model
pub kinesis_data_stream_destinations: ::std::option::Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn kinesis_data_stream_destinations(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>> {
    &self.kinesis_data_stream_destinations
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeKinesisStreamingDestinationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeKinesisStreamingDestinationOutput`](crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutput).
    pub fn builder() -> crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder {
        crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationOutputBuilder::default()
    }
}

/// A builder for [`DescribeKinesisStreamingDestinationOutput`](crate::operation::operation::DescribeKinesisStreamingDestinationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeKinesisStreamingDestinationOutputBuilder {
    pub(crate) kinesis_data_stream_destinations: ::std::option::Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn kinesis_data_stream_destinations(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>>) -> Self {
    self.kinesis_data_stream_destinations = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_kinesis_data_stream_destinations(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>>) -> Self {
    self.kinesis_data_stream_destinations = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_kinesis_data_stream_destinations(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::KinesisDataStreamDestination>> {
    &self.kinesis_data_stream_destinations
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`DescribeKinesisStreamingDestinationOutput`](crate::operation::operation::DescribeKinesisStreamingDestinationOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput {
            kinesis_data_stream_destinations: self.kinesis_data_stream_destinations,
table_name: self.table_name,
        })
    }
}
