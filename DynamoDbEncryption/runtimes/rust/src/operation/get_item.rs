// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetItem`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetItem;
impl GetItem {
    /// Creates a new `GetItem`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::get_item::GetItemInput,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::get_item::_get_item_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).GetItem(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::get_item::_get_item_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::get_item::_get_item_output::GetItemOutput;

pub use crate::operation::get_item::_get_item_input::GetItemInput;

pub(crate) mod _get_item_output;

pub(crate) mod _get_item_input;

/// Builders
pub mod builders;