// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `DescribeCustomKeyStores`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeCustomKeyStores;
impl DescribeCustomKeyStores {
    /// Creates a new `DescribeCustomKeyStores`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresRequest,
    ) -> ::std::result::Result<
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::describe_custom_key_stores::_describe_custom_key_stores_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).DescribeCustomKeyStores(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::describe_custom_key_stores::_describe_custom_key_stores_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::describe_custom_key_stores::_describe_custom_key_stores_response::DescribeCustomKeyStoresResponse;

pub use crate::operation::describe_custom_key_stores::_describe_custom_key_stores_request::DescribeCustomKeyStoresRequest;

pub(crate) mod _describe_custom_key_stores_response;

pub(crate) mod _describe_custom_key_stores_request;

/// Builders
pub mod builders;