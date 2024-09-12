// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeKinesisStreamingDestination`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeKinesisStreamingDestination;
impl DescribeKinesisStreamingDestination {
    /// Creates a new `DescribeKinesisStreamingDestination`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationInput,
    ) -> ::std::result::Result<
        crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeKinesisStreamingDestination(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_output::DescribeKinesisStreamingDestinationOutput;

pub use crate::operation::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_input::DescribeKinesisStreamingDestinationInput;

pub(crate) mod _describe_kinesis_streaming_destination_output;

pub(crate) mod _describe_kinesis_streaming_destination_input;

/// Builders
pub mod builders;
