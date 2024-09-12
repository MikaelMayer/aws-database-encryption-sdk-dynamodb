// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::list_contributor_insights::_list_contributor_insights_output::ListContributorInsightsOutputBuilder;

pub use crate::operation::list_contributor_insights::_list_contributor_insights_input::ListContributorInsightsInputBuilder;

impl ListContributorInsightsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::list_contributor_insights::ListContributorInsightsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.list_contributor_insights();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListContributorInsights`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListContributorInsightsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::list_contributor_insights::builders::ListContributorInsightsInputBuilder,
}
impl ListContributorInsightsFluentBuilder {
    /// Creates a new `ListContributorInsights`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ListContributorInsights as a reference.
    pub fn as_input(&self) -> &crate::operation::list_contributor_insights::builders::ListContributorInsightsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_contributor_insights::ListContributorInsightsOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::list_contributor_insights::ListContributorInsights::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn max_results(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.inner = self.inner.max_results(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_max_results(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.inner = self.inner.set_max_results(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_max_results(&self) -> &::std::option::Option<::std::primitive::i32> {
    self.inner.get_max_results()
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.next_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_next_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_next_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
}
