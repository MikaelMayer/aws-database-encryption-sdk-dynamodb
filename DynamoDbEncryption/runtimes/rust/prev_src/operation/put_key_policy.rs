// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `PutKeyPolicy`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutKeyPolicy;
impl PutKeyPolicy {
    /// Creates a new `PutKeyPolicy`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::put_key_policy::PutKeyPolicyRequest,
    ) -> ::std::result::Result<
        crate::operation::put_key_policy::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::put_key_policy::_put_key_policy_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).PutKeyPolicy(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::put_key_policy::_put_key_policy_output::from_dafny(
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

pub use crate::operation::put_key_policy::_unit::Unit;

pub use crate::operation::put_key_policy::_put_key_policy_request::PutKeyPolicyRequest;

pub(crate) mod _unit;

pub(crate) mod _put_key_policy_request;

/// Builders
pub mod builders;
