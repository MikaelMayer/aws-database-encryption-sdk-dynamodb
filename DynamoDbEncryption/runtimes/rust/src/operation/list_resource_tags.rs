// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListResourceTags`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListResourceTags;
impl ListResourceTags {
    /// Creates a new `ListResourceTags`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_resource_tags::ListResourceTagsRequest,
    ) -> ::std::result::Result<
        crate::operation::list_resource_tags::ListResourceTagsResponse,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_resource_tags::_list_resource_tags_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListResourceTags(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_resource_tags::_list_resource_tags_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::list_resource_tags::_list_resource_tags_response::ListResourceTagsResponse;

pub use crate::operation::list_resource_tags::_list_resource_tags_request::ListResourceTagsRequest;

pub(crate) mod _list_resource_tags_response;

pub(crate) mod _list_resource_tags_request;

/// Builders
pub mod builders;
