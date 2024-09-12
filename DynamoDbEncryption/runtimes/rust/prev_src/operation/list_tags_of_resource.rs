// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListTagsOfResource`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListTagsOfResource;
impl ListTagsOfResource {
    /// Creates a new `ListTagsOfResource`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_tags_of_resource::ListTagsOfResourceInput,
    ) -> ::std::result::Result<
        crate::operation::list_tags_of_resource::ListTagsOfResourceOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_tags_of_resource::_list_tags_of_resource_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListTagsOfResource(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_tags_of_resource::_list_tags_of_resource_output::from_dafny(
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

pub use crate::operation::list_tags_of_resource::_list_tags_of_resource_output::ListTagsOfResourceOutput;

pub use crate::operation::list_tags_of_resource::_list_tags_of_resource_input::ListTagsOfResourceInput;

pub(crate) mod _list_tags_of_resource_output;

pub(crate) mod _list_tags_of_resource_input;

/// Builders
pub mod builders;
