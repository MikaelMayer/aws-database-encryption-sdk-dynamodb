// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `EnableKey`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EnableKey;
impl EnableKey {
    /// Creates a new `EnableKey`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::enable_key::EnableKeyRequest,
    ) -> ::std::result::Result<
        crate::operation::enable_key::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::enable_key::_enable_key_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).EnableKey(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::enable_key::_enable_key_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::enable_key::_unit::Unit;

pub use crate::operation::enable_key::_enable_key_request::EnableKeyRequest;

pub(crate) mod _unit;

pub(crate) mod _enable_key_request;

/// Builders
pub mod builders;
