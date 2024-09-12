// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableKinesisStreamingDestinationOutput {
    #[allow(missing_docs)] // documentation missing in model
pub destination_status: ::std::option::Option<dynamodb::types::DestinationStatus>,
#[allow(missing_docs)] // documentation missing in model
pub stream_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl EnableKinesisStreamingDestinationOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn destination_status(&self) -> &::std::option::Option<dynamodb::types::DestinationStatus> {
    &self.destination_status
}
#[allow(missing_docs)] // documentation missing in model
pub fn stream_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.stream_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl EnableKinesisStreamingDestinationOutput {
    /// Creates a new builder-style object to manufacture [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutput).
    pub fn builder() -> crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder {
        crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder::default()
    }
}

/// A builder for [`EnableKinesisStreamingDestinationOutput`](crate::operation::operation::EnableKinesisStreamingDestinationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnableKinesisStreamingDestinationOutputBuilder {
    pub(crate) destination_status: ::std::option::Option<dynamodb::types::DestinationStatus>,
pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl EnableKinesisStreamingDestinationOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn destination_status(mut self, input: impl ::std::convert::Into<dynamodb::types::DestinationStatus>) -> Self {
    self.destination_status = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_destination_status(mut self, input: ::std::option::Option<dynamodb::types::DestinationStatus>) -> Self {
    self.destination_status = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_destination_status(&self) -> &::std::option::Option<dynamodb::types::DestinationStatus> {
    &self.destination_status
}
#[allow(missing_docs)] // documentation missing in model
pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.stream_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.stream_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.stream_arn
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
    /// Consumes the builder and constructs a [`EnableKinesisStreamingDestinationOutput`](crate::operation::operation::EnableKinesisStreamingDestinationOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput {
            destination_status: self.destination_status,
stream_arn: self.stream_arn,
table_name: self.table_name,
        })
    }
}
