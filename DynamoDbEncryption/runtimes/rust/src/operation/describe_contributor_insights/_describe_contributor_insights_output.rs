// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub contributor_insights_rule_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub contributor_insights_status: ::std::option::Option<dynamodb::types::ContributorInsightsStatus>,
#[allow(missing_docs)] // documentation missing in model
pub failure_exception: ::std::option::Option<dynamodb::types::FailureException>,
#[allow(missing_docs)] // documentation missing in model
pub index_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub last_update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_rule_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.contributor_insights_rule_list
}
#[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_status(&self) -> &::std::option::Option<dynamodb::types::ContributorInsightsStatus> {
    &self.contributor_insights_status
}
#[allow(missing_docs)] // documentation missing in model
pub fn failure_exception(&self) -> &::std::option::Option<dynamodb::types::FailureException> {
    &self.failure_exception
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.index_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_update_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.last_update_date_time
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DescribeContributorInsightsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeContributorInsightsOutput`](crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsOutput).
    pub fn builder() -> crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsOutputBuilder {
        crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsOutputBuilder::default()
    }
}

/// A builder for [`DescribeContributorInsightsOutput`](crate::operation::operation::DescribeContributorInsightsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeContributorInsightsOutputBuilder {
    pub(crate) contributor_insights_rule_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) contributor_insights_status: ::std::option::Option<dynamodb::types::ContributorInsightsStatus>,
pub(crate) failure_exception: ::std::option::Option<dynamodb::types::FailureException>,
pub(crate) index_name: ::std::option::Option<::std::string::String>,
pub(crate) last_update_date_time: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeContributorInsightsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_rule_list(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.contributor_insights_rule_list = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_contributor_insights_rule_list(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.contributor_insights_rule_list = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_contributor_insights_rule_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.contributor_insights_rule_list
}
#[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_status(mut self, input: impl ::std::convert::Into<dynamodb::types::ContributorInsightsStatus>) -> Self {
    self.contributor_insights_status = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_contributor_insights_status(mut self, input: ::std::option::Option<dynamodb::types::ContributorInsightsStatus>) -> Self {
    self.contributor_insights_status = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_contributor_insights_status(&self) -> &::std::option::Option<dynamodb::types::ContributorInsightsStatus> {
    &self.contributor_insights_status
}
#[allow(missing_docs)] // documentation missing in model
pub fn failure_exception(mut self, input: impl ::std::convert::Into<dynamodb::types::FailureException>) -> Self {
    self.failure_exception = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_failure_exception(mut self, input: ::std::option::Option<dynamodb::types::FailureException>) -> Self {
    self.failure_exception = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_failure_exception(&self) -> &::std::option::Option<dynamodb::types::FailureException> {
    &self.failure_exception
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.index_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.index_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.index_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_update_date_time(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.last_update_date_time = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_update_date_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.last_update_date_time = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_update_date_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.last_update_date_time
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`DescribeContributorInsightsOutput`](crate::operation::operation::DescribeContributorInsightsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput {
            contributor_insights_rule_list: self.contributor_insights_rule_list,
contributor_insights_status: self.contributor_insights_status,
failure_exception: self.failure_exception,
index_name: self.index_name,
last_update_date_time: self.last_update_date_time,
table_name: self.table_name,
        })
    }
}
