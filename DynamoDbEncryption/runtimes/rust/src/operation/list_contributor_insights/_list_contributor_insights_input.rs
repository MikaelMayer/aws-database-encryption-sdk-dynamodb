// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListContributorInsightsInput {
    #[allow(missing_docs)] // documentation missing in model
pub max_results: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl ListContributorInsightsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn max_results(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.max_results
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl ListContributorInsightsInput {
    /// Creates a new builder-style object to manufacture [`ListContributorInsightsInput`](crate::operation::list_contributor_insights::builders::ListContributorInsightsInput).
    pub fn builder() -> crate::operation::list_contributor_insights::builders::ListContributorInsightsInputBuilder {
        crate::operation::list_contributor_insights::builders::ListContributorInsightsInputBuilder::default()
    }
}

/// A builder for [`ListContributorInsightsInput`](crate::operation::operation::ListContributorInsightsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListContributorInsightsInputBuilder {
    pub(crate) max_results: ::std::option::Option<::std::primitive::i32>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl ListContributorInsightsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn max_results(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.max_results = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_max_results(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.max_results = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_max_results(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.max_results
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
    /// Consumes the builder and constructs a [`ListContributorInsightsInput`](crate::operation::operation::ListContributorInsightsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_contributor_insights::ListContributorInsightsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_contributor_insights::ListContributorInsightsInput {
            max_results: self.max_results,
next_token: self.next_token,
table_name: self.table_name,
        })
    }
}
