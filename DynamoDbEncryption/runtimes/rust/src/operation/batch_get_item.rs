// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `BatchGetItem`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BatchGetItem;
impl BatchGetItem {
    /// Creates a new `BatchGetItem`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::batch_get_item::BatchGetItemInput,
    ) -> ::std::result::Result<
        crate::operation::batch_get_item::BatchGetItemOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::batch_get_item::_batch_get_item_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).BatchGetItem(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::batch_get_item::_batch_get_item_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::batch_get_item::_batch_get_item_output::BatchGetItemOutput;

pub use crate::operation::batch_get_item::_batch_get_item_input::BatchGetItemInput;

pub(crate) mod _batch_get_item_output;

pub(crate) mod _batch_get_item_input;

/// Builders
pub mod builders;
