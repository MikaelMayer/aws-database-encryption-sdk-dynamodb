// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListGrants`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListGrants;
impl ListGrants {
    /// Creates a new `ListGrants`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_grants::ListGrantsRequest,
    ) -> ::std::result::Result<
        crate::operation::list_grants::ListGrantsResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_grants::_list_grants_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListGrants(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_grants::_list_grants_output::from_dafny(
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

pub use crate::operation::list_grants::_list_grants_response::ListGrantsResponse;

pub use crate::operation::list_grants::_list_grants_request::ListGrantsRequest;

pub(crate) mod _list_grants_response;

pub(crate) mod _list_grants_request;

/// Builders
pub mod builders;
