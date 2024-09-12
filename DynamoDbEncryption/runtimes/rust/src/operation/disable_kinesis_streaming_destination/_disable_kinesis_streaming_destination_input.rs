// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableKinesisStreamingDestinationInput {
    #[allow(missing_docs)] // documentation missing in model
pub stream_arn: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DisableKinesisStreamingDestinationInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn stream_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.stream_arn
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DisableKinesisStreamingDestinationInput {
    /// Creates a new builder-style object to manufacture [`DisableKinesisStreamingDestinationInput`](crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInput).
    pub fn builder() -> crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder {
        crate::operation::disable_kinesis_streaming_destination::builders::DisableKinesisStreamingDestinationInputBuilder::default()
    }
}

/// A builder for [`DisableKinesisStreamingDestinationInput`](crate::operation::operation::DisableKinesisStreamingDestinationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisableKinesisStreamingDestinationInputBuilder {
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DisableKinesisStreamingDestinationInputBuilder {
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
    /// Consumes the builder and constructs a [`DisableKinesisStreamingDestinationInput`](crate::operation::operation::DisableKinesisStreamingDestinationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationInput {
            stream_arn: self.stream_arn,
table_name: self.table_name,
        })
    }
}
