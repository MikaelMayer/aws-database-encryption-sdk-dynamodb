// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Scan`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Scan;
impl Scan {
    /// Creates a new `Scan`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::scan::ScanInput,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::scan::_scan_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Scan(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::scan::_scan_output::from_dafny(
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

pub use crate::operation::scan::_scan_output::ScanOutput;

pub use crate::operation::scan::_scan_input::ScanInput;

pub(crate) mod _scan_output;

pub(crate) mod _scan_input;

/// Builders
pub mod builders;
