// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ExecuteStatement`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ExecuteStatement;
impl ExecuteStatement {
    /// Creates a new `ExecuteStatement`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::execute_statement::ExecuteStatementInput,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::execute_statement::_execute_statement_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ExecuteStatement(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::execute_statement::_execute_statement_output::from_dafny(
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

pub use crate::operation::execute_statement::_execute_statement_output::ExecuteStatementOutput;

pub use crate::operation::execute_statement::_execute_statement_input::ExecuteStatementInput;

pub(crate) mod _execute_statement_output;

pub(crate) mod _execute_statement_input;

/// Builders
pub mod builders;
