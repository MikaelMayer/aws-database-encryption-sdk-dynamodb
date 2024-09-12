// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListAliases`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListAliases;
impl ListAliases {
    /// Creates a new `ListAliases`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_aliases::ListAliasesRequest,
    ) -> ::std::result::Result<
        crate::operation::list_aliases::ListAliasesResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_aliases::_list_aliases_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListAliases(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_aliases::_list_aliases_output::from_dafny(
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

pub use crate::operation::list_aliases::_list_aliases_response::ListAliasesResponse;

pub use crate::operation::list_aliases::_list_aliases_request::ListAliasesRequest;

pub(crate) mod _list_aliases_response;

pub(crate) mod _list_aliases_request;

/// Builders
pub mod builders;
