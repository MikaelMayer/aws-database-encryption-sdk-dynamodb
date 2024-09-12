// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `UpdateContributorInsights`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateContributorInsights;
impl UpdateContributorInsights {
    /// Creates a new `UpdateContributorInsights`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::update_contributor_insights::UpdateContributorInsightsInput,
    ) -> ::std::result::Result<
        crate::operation::update_contributor_insights::UpdateContributorInsightsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::update_contributor_insights::_update_contributor_insights_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).UpdateContributorInsights(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::update_contributor_insights::_update_contributor_insights_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::update_contributor_insights::_update_contributor_insights_output::UpdateContributorInsightsOutput;

pub use crate::operation::update_contributor_insights::_update_contributor_insights_input::UpdateContributorInsightsInput;

pub(crate) mod _update_contributor_insights_output;

pub(crate) mod _update_contributor_insights_input;

/// Builders
pub mod builders;
