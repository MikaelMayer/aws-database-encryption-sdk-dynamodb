// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeTableReplicaAutoScaling`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeTableReplicaAutoScaling;
impl DescribeTableReplicaAutoScaling {
    /// Creates a new `DescribeTableReplicaAutoScaling`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingInput,
    ) -> ::std::result::Result<
        crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeTableReplicaAutoScaling(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_output::DescribeTableReplicaAutoScalingOutput;

pub use crate::operation::describe_table_replica_auto_scaling::_describe_table_replica_auto_scaling_input::DescribeTableReplicaAutoScalingInput;

pub(crate) mod _describe_table_replica_auto_scaling_output;

pub(crate) mod _describe_table_replica_auto_scaling_input;

/// Builders
pub mod builders;
