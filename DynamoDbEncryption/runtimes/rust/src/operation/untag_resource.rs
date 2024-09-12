// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UntagResource`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UntagResource;
impl UntagResource {
    /// Creates a new `UntagResource`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::untag_resource::UntagResourceInput,
    ) -> ::std::result::Result<
        crate::operation::untag_resource::Unit,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::untag_resource::_untag_resource_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UntagResource(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::untag_resource::_untag_resource_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::untag_resource::_unit::Unit;

pub use crate::operation::untag_resource::_untag_resource_input::UntagResourceInput;

pub(crate) mod _unit;

pub(crate) mod _untag_resource_input;

/// Builders
pub mod builders;
