// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeKinesisStreamingDestinationInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeKinesisStreamingDestinationInput {
    /// Creates a new builder-style object to manufacture [`DescribeKinesisStreamingDestinationInput`](crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInput).
    pub fn builder() -> crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder {
        crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder::default()
    }
}

/// A builder for [`DescribeKinesisStreamingDestinationInput`](crate::operation::operation::DescribeKinesisStreamingDestinationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeKinesisStreamingDestinationInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeKinesisStreamingDestinationInputBuilder {
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
    /// Consumes the builder and constructs a [`DescribeKinesisStreamingDestinationInput`](crate::operation::operation::DescribeKinesisStreamingDestinationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput {
            table_name: self.table_name,
        })
    }
}
