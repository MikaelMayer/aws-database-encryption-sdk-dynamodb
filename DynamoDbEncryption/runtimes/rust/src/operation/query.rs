// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `Query`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Query;
impl Query {
    /// Creates a new `Query`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::query::QueryInput,
    ) -> ::std::result::Result<
        crate::operation::query::QueryOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::query::_query_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).Query(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::query::_query_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::query::_query_output::QueryOutput;

pub use crate::operation::query::_query_input::QueryInput;

pub(crate) mod _query_output;

pub(crate) mod _query_input;

/// Builders
pub mod builders;
