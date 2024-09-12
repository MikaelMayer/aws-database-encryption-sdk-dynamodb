// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `BatchWriteItem`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BatchWriteItem;
impl BatchWriteItem {
    /// Creates a new `BatchWriteItem`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::batch_write_item::BatchWriteItemInput,
    ) -> ::std::result::Result<
        crate::operation::batch_write_item::BatchWriteItemOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::batch_write_item::_batch_write_item_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).BatchWriteItem(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::batch_write_item::_batch_write_item_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::batch_write_item::_batch_write_item_output::BatchWriteItemOutput;

pub use crate::operation::batch_write_item::_batch_write_item_input::BatchWriteItemInput;

pub(crate) mod _batch_write_item_output;

pub(crate) mod _batch_write_item_input;

/// Builders
pub mod builders;
