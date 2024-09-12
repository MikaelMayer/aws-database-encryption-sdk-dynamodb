// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EnableKinesisStreamingDestination`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EnableKinesisStreamingDestination;
impl EnableKinesisStreamingDestination {
    /// Creates a new `EnableKinesisStreamingDestination`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationInput,
    ) -> ::std::result::Result<
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).EnableKinesisStreamingDestination(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_output::from_dafny(
                    inner_result.value().clone(),
                ),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_output::EnableKinesisStreamingDestinationOutput;

pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_input::EnableKinesisStreamingDestinationInput;

pub(crate) mod _enable_kinesis_streaming_destination_output;

pub(crate) mod _enable_kinesis_streaming_destination_input;

/// Builders
pub mod builders;
