// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DisableKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DisableKey;
impl DisableKey {
    /// Creates a new `DisableKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::disable_key::DisableKeyRequest,
    ) -> ::std::result::Result<
        crate::operation::disable_key::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::disable_key::_disable_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DisableKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::disable_key::_disable_key_output::from_dafny(
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

pub use crate::operation::disable_key::_unit::Unit;

pub use crate::operation::disable_key::_disable_key_request::DisableKeyRequest;

pub(crate) mod _unit;

pub(crate) mod _disable_key_request;

/// Builders
pub mod builders;
