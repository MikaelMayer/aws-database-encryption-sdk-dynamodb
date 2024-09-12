// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub contributor_insights_status: ::std::option::Option<dynamodb::types::ContributorInsightsStatus>,
#[allow(missing_docs)] // documentation missing in model
pub index_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateContributorInsightsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn contributor_insights_status(&self) -> &::std::option::Option<dynamodb::types::ContributorInsightsStatus> {
    &self.contributor_insights_status
}
#[allow(missing_docs)] // documentation missing in model
pub fn index_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.index_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl UpdateContributorInsightsOutput {
    /// Creates a new builder-style object to manufacture [`UpdateContributorInsightsOutput`](crate::operation::update_contributor_insights::builders::UpdateContributorInsightsOutput).
    pub fn builder() -> crate::operation::update_contributor_insights::builders::UpdateContributorInsightsOutputBuilder {
        crate::operation::update_contributor_insights::builders::UpdateContributorInsightsOutputBuilder::default()
    }
}

/// A builder for [`UpdateContributorInsightsOutput`](crate::operation::operation::UpdateContributorInsightsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateContributorInsightsOutputBuilder {
    pub(crate) contributor_insights_status: ::std::option::Option<dynamodb::types::ContributorInsightsStatus>,
pub(crate) index_name: ::std::option::Option<::std::string::String>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl UpdateContributorInsightsOutputBuilder {
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
    /// Consumes the builder and constructs a [`UpdateContributorInsightsOutput`](crate::operation::operation::UpdateContributorInsightsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_contributor_insights::UpdateContributorInsightsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_contributor_insights::UpdateContributorInsightsOutput {
            contributor_insights_status: self.contributor_insights_status,
index_name: self.index_name,
table_name: self.table_name,
        })
    }
}
