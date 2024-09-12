// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `TagResource`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TagResource;
impl TagResource {
    /// Creates a new `TagResource`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::tag_resource::TagResourceRequest,
    ) -> ::std::result::Result<
        crate::operation::tag_resource::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::tag_resource::_tag_resource_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).TagResource(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::tag_resource::_tag_resource_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::tag_resource::_unit::Unit;

pub use crate::operation::tag_resource::_tag_resource_request::TagResourceRequest;

pub(crate) mod _unit;

pub(crate) mod _tag_resource_request;

/// Builders
pub mod builders;
