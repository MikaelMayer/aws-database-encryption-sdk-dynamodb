// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DisableKinesisStreamingDestination`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DisableKinesisStreamingDestination;
impl DisableKinesisStreamingDestination {
    /// Creates a new `DisableKinesisStreamingDestination`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationInput,
    ) -> ::std::result::Result<
        crate::operation::disable_kinesis_streaming_destination::DisableKinesisStreamingDestinationOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DisableKinesisStreamingDestination(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_output::from_dafny(
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

pub use crate::operation::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_output::DisableKinesisStreamingDestinationOutput;

pub use crate::operation::disable_kinesis_streaming_destination::_disable_kinesis_streaming_destination_input::DisableKinesisStreamingDestinationInput;

pub(crate) mod _disable_kinesis_streaming_destination_output;

pub(crate) mod _disable_kinesis_streaming_destination_input;

/// Builders
pub mod builders;
