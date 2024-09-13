// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `BatchExecuteStatement`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BatchExecuteStatement;
impl BatchExecuteStatement {
    /// Creates a new `BatchExecuteStatement`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::batch_execute_statement::BatchExecuteStatementInput,
    ) -> ::std::result::Result<
        crate::operation::batch_execute_statement::BatchExecuteStatementOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::batch_execute_statement::_batch_execute_statement_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).BatchExecuteStatement(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::batch_execute_statement::_batch_execute_statement_output::from_dafny(
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

pub use crate::operation::batch_execute_statement::_batch_execute_statement_output::BatchExecuteStatementOutput;

pub use crate::operation::batch_execute_statement::_batch_execute_statement_input::BatchExecuteStatementInput;

pub(crate) mod _batch_execute_statement_output;

pub(crate) mod _batch_execute_statement_input;

/// Builders
pub mod builders;