// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateContributorInsights`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`contributor_insights_action(impl Into<Option<dynamodb::types::ContributorInsightsAction>>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::contributor_insights_action) / [`set_contributor_insights_action(Option<dynamodb::types::ContributorInsightsAction>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::set_contributor_insights_action): (undocumented)<br>
    ///   - [`index_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::index_name) / [`set_index_name(Option<::std::string::String>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::set_index_name): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`UpdateContributorInsightsOutput`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput) with field(s):
    ///   - [`contributor_insights_status(Option<dynamodb::types::ContributorInsightsStatus>)`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput::contributor_insights_status): (undocumented)
    ///   - [`index_name(Option<::std::string::String>)`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput::index_name): (undocumented)
    ///   - [`table_name(Option<::std::string::String>)`](crate::operation::update_contributor_insights::UpdateContributorInsightsOutput::table_name): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateContributorInsightsError>`](crate::operation::update_contributor_insights::UpdateContributorInsightsError)
    pub fn update_contributor_insights(&self) -> crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder {
        crate::operation::update_contributor_insights::builders::UpdateContributorInsightsFluentBuilder::new(self.clone())
    }
}
