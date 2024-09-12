// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListContributorInsights`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::max_results) / [`set_max_results(Option<::std::primitive::i32>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::set_max_results): (undocumented)<br>
    ///   - [`next_token(impl Into<Option<::std::string::String>>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::next_token) / [`set_next_token(Option<::std::string::String>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::set_next_token): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`ListContributorInsightsOutput`](crate::operation::list_contributor_insights::ListContributorInsightsOutput) with field(s):
    ///   - [`contributor_insights_summaries(Option<::std::vec::Vec<dynamodb::types::ContributorInsightsSummary>>)`](crate::operation::list_contributor_insights::ListContributorInsightsOutput::contributor_insights_summaries): (undocumented)
    ///   - [`next_token(Option<::std::string::String>)`](crate::operation::list_contributor_insights::ListContributorInsightsOutput::next_token): (undocumented)
    /// - On failure, responds with [`SdkError<ListContributorInsightsError>`](crate::operation::list_contributor_insights::ListContributorInsightsError)
    pub fn list_contributor_insights(&self) -> crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder {
        crate::operation::list_contributor_insights::builders::ListContributorInsightsFluentBuilder::new(self.clone())
    }
}
