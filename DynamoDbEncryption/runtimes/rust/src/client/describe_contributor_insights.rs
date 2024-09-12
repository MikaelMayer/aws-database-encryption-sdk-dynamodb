// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeContributorInsights`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`index_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder::index_name) / [`set_index_name(Option<::std::string::String>)`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder::set_index_name): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeContributorInsightsOutput`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput) with field(s):
    ///   - [`contributor_insights_rule_list(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::contributor_insights_rule_list): (undocumented)
    ///   - [`contributor_insights_status(Option<dynamodb::types::ContributorInsightsStatus>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::contributor_insights_status): (undocumented)
    ///   - [`failure_exception(Option<dynamodb::types::FailureException>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::failure_exception): (undocumented)
    ///   - [`index_name(Option<::std::string::String>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::index_name): (undocumented)
    ///   - [`last_update_date_time(Option<::aws_smithy_types::DateTime>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::last_update_date_time): (undocumented)
    ///   - [`table_name(Option<::std::string::String>)`](crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput::table_name): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeContributorInsightsError>`](crate::operation::describe_contributor_insights::DescribeContributorInsightsError)
    pub fn describe_contributor_insights(&self) -> crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder {
        crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsFluentBuilder::new(self.clone())
    }
}
