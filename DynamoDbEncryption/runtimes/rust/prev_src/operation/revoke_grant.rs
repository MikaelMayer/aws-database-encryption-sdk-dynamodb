// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `RevokeGrant`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RevokeGrant;
impl RevokeGrant {
    /// Creates a new `RevokeGrant`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::revoke_grant::RevokeGrantRequest,
    ) -> ::std::result::Result<
        crate::operation::revoke_grant::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::revoke_grant::_revoke_grant_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).RevokeGrant(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::revoke_grant::_revoke_grant_output::from_dafny(
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

pub use crate::operation::revoke_grant::_unit::Unit;

pub use crate::operation::revoke_grant::_revoke_grant_request::RevokeGrantRequest;

pub(crate) mod _unit;

pub(crate) mod _revoke_grant_request;

/// Builders
pub mod builders;
