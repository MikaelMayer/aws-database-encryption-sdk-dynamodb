// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `RetireGrant`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RetireGrant;
impl RetireGrant {
    /// Creates a new `RetireGrant`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::retire_grant::RetireGrantRequest,
    ) -> ::std::result::Result<
        crate::operation::retire_grant::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::retire_grant::_retire_grant_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).RetireGrant(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::retire_grant::_retire_grant_output::from_dafny(
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

pub use crate::operation::retire_grant::_unit::Unit;

pub use crate::operation::retire_grant::_retire_grant_request::RetireGrantRequest;

pub(crate) mod _unit;

pub(crate) mod _retire_grant_request;

/// Builders
pub mod builders;
