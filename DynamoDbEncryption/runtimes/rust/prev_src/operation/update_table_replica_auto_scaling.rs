// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateTableReplicaAutoScaling`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateTableReplicaAutoScaling;
impl UpdateTableReplicaAutoScaling {
    /// Creates a new `UpdateTableReplicaAutoScaling`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingInput,
    ) -> ::std::result::Result<
        crate::operation::update_table_replica_auto_scaling::UpdateTableReplicaAutoScalingOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateTableReplicaAutoScaling(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_output::from_dafny(
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

pub use crate::operation::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_output::UpdateTableReplicaAutoScalingOutput;

pub use crate::operation::update_table_replica_auto_scaling::_update_table_replica_auto_scaling_input::UpdateTableReplicaAutoScalingInput;

pub(crate) mod _update_table_replica_auto_scaling_output;

pub(crate) mod _update_table_replica_auto_scaling_input;

/// Builders
pub mod builders;
