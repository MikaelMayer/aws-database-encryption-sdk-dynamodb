// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListKeys`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListKeys;
impl ListKeys {
    /// Creates a new `ListKeys`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_keys::ListKeysRequest,
    ) -> ::std::result::Result<
        crate::operation::list_keys::ListKeysResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_keys::_list_keys_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListKeys(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_keys::_list_keys_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::list_keys::_list_keys_response::ListKeysResponse;

pub use crate::operation::list_keys::_list_keys_request::ListKeysRequest;

pub(crate) mod _list_keys_response;

pub(crate) mod _list_keys_request;

/// Builders
pub mod builders;
