// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetKeyStoreInfo`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetKeyStoreInfo;
impl GetKeyStoreInfo {
    /// Creates a new `GetKeyStoreInfo`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::material_providers::client::Client,
        input: crate::material_providers::operation::get_key_store_info::Unit,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::material_providers::types::error::Error,
    > {
                let inner_input = crate::material_providers::conversions::get_key_store_info::_get_key_store_info_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetKeyStoreInfo(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::Wrappers::Result::Success { .. }
        ) {
            Ok(
                crate::material_providers::conversions::get_key_store_info::_get_key_store_info_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::material_providers::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::material_providers::operation::get_key_store_info::_get_key_store_info_output::GetKeyStoreInfoOutput;

pub use crate::material_providers::operation::get_key_store_info::_unit::Unit;

pub(crate) mod _get_key_store_info_output;

pub(crate) mod _unit;

/// Builders
pub mod builders;