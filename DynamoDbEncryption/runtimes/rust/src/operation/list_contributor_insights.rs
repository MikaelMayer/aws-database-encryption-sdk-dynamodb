// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `ListContributorInsights`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListContributorInsights;
impl ListContributorInsights {
    /// Creates a new `ListContributorInsights`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::list_contributor_insights::ListContributorInsightsInput,
    ) -> ::std::result::Result<
        crate::operation::list_contributor_insights::ListContributorInsightsOutput,
        crate::types::error::Error,
    > {
                let inner_input = crate::conversions::list_contributor_insights::_list_contributor_insights_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).ListContributorInsights(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::list_contributor_insights::_list_contributor_insights_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::list_contributor_insights::_list_contributor_insights_output::ListContributorInsightsOutput;

pub use crate::operation::list_contributor_insights::_list_contributor_insights_input::ListContributorInsightsInput;

pub(crate) mod _list_contributor_insights_output;

pub(crate) mod _list_contributor_insights_input;

/// Builders
pub mod builders;
