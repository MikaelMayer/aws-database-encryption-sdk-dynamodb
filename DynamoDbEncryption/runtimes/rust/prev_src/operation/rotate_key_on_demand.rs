// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `RotateKeyOnDemand`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RotateKeyOnDemand;
impl RotateKeyOnDemand {
    /// Creates a new `RotateKeyOnDemand`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::rotate_key_on_demand::RotateKeyOnDemandRequest,
    ) -> ::std::result::Result<
        crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::rotate_key_on_demand::_rotate_key_on_demand_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).RotateKeyOnDemand(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::rotate_key_on_demand::_rotate_key_on_demand_output::from_dafny(
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

pub use crate::operation::rotate_key_on_demand::_rotate_key_on_demand_response::RotateKeyOnDemandResponse;

pub use crate::operation::rotate_key_on_demand::_rotate_key_on_demand_request::RotateKeyOnDemandRequest;

pub(crate) mod _rotate_key_on_demand_response;

pub(crate) mod _rotate_key_on_demand_request;

/// Builders
pub mod builders;
