// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ExportTableToPointInTime`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ExportTableToPointInTime;
impl ExportTableToPointInTime {
    /// Creates a new `ExportTableToPointInTime`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput,
    ) -> ::std::result::Result<
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::export_table_to_point_in_time::_export_table_to_point_in_time_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ExportTableToPointInTime(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::export_table_to_point_in_time::_export_table_to_point_in_time_output::from_dafny(
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

pub use crate::operation::export_table_to_point_in_time::_export_table_to_point_in_time_output::ExportTableToPointInTimeOutput;

pub use crate::operation::export_table_to_point_in_time::_export_table_to_point_in_time_input::ExportTableToPointInTimeInput;

pub(crate) mod _export_table_to_point_in_time_output;

pub(crate) mod _export_table_to_point_in_time_input;

/// Builders
pub mod builders;
