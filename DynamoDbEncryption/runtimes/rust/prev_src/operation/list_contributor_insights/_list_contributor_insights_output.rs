// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub contributor_insights_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>> {
    &self.contributor_insights_summaries
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
}
impl ListContributorInsightsOutput {
    /// Creates a new builder-style object to manufacture [`ListContributorInsightsOutput`](crate::operation::list_contributor_insights::builders::ListContributorInsightsOutput).
    pub fn builder() -> crate::operation::list_contributor_insights::builders::ListContributorInsightsOutputBuilder {
        crate::operation::list_contributor_insights::builders::ListContributorInsightsOutputBuilder::default()
    }
}

/// A builder for [`ListContributorInsightsOutput`](crate::operation::operation::ListContributorInsightsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListContributorInsightsOutputBuilder {
    pub(crate) contributor_insights_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListContributorInsightsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_summaries(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>>) -> Self {
    self.contributor_insights_summaries = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_contributor_insights_summaries(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>>) -> Self {
    self.contributor_insights_summaries = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_contributor_insights_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>> {
    &self.contributor_insights_summaries
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.next_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.next_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
    /// Consumes the builder and constructs a [`ListContributorInsightsOutput`](crate::operation::operation::ListContributorInsightsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_contributor_insights::ListContributorInsightsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_contributor_insights::ListContributorInsightsOutput {
            contributor_insights_summaries: self.contributor_insights_summaries,
next_token: self.next_token,
        })
    }
}
