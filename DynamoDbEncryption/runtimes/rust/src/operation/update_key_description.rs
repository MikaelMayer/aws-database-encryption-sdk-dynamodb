// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateKeyDescription`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateKeyDescription;
impl UpdateKeyDescription {
    /// Creates a new `UpdateKeyDescription`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_key_description::UpdateKeyDescriptionRequest,
    ) -> ::std::result::Result<
        crate::operation::update_key_description::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_key_description::_update_key_description_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateKeyDescription(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_key_description::_update_key_description_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::update_key_description::_unit::Unit;

pub use crate::operation::update_key_description::_update_key_description_request::UpdateKeyDescriptionRequest;

pub(crate) mod _unit;

pub(crate) mod _update_key_description_request;

/// Builders
pub mod builders;
