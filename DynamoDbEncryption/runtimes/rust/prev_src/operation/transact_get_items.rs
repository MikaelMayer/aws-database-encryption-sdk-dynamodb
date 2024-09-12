// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `TransactGetItems`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransactGetItems;
impl TransactGetItems {
    /// Creates a new `TransactGetItems`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::transact_get_items::TransactGetItemsInput,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::transact_get_items::_transact_get_items_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).TransactGetItems(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::transact_get_items::_transact_get_items_output::from_dafny(
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

pub use crate::operation::transact_get_items::_transact_get_items_output::TransactGetItemsOutput;

pub use crate::operation::transact_get_items::_transact_get_items_input::TransactGetItemsInput;

pub(crate) mod _transact_get_items_output;

pub(crate) mod _transact_get_items_input;

/// Builders
pub mod builders;
