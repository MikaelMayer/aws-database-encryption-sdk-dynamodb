// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdatePrimaryRegion`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdatePrimaryRegion;
impl UpdatePrimaryRegion {
    /// Creates a new `UpdatePrimaryRegion`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_primary_region::UpdatePrimaryRegionRequest,
    ) -> ::std::result::Result<
        crate::operation::update_primary_region::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_primary_region::_update_primary_region_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdatePrimaryRegion(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_primary_region::_update_primary_region_output::from_dafny(
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

pub use crate::operation::update_primary_region::_unit::Unit;

pub use crate::operation::update_primary_region::_update_primary_region_request::UpdatePrimaryRegionRequest;

pub(crate) mod _unit;

pub(crate) mod _update_primary_region_request;

/// Builders
pub mod builders;
