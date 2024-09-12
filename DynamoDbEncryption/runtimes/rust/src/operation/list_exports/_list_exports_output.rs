// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListExportsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub export_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::ExportSummary>>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ExportSummary>> {
    &self.export_summaries
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
}
impl ListExportsOutput {
    /// Creates a new builder-style object to manufacture [`ListExportsOutput`](crate::operation::list_exports::builders::ListExportsOutput).
    pub fn builder() -> crate::operation::list_exports::builders::ListExportsOutputBuilder {
        crate::operation::list_exports::builders::ListExportsOutputBuilder::default()
    }
}

/// A builder for [`ListExportsOutput`](crate::operation::operation::ListExportsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListExportsOutputBuilder {
    pub(crate) export_summaries: ::std::option::Option<::std::vec::Vec<dynamodb::types::ExportSummary>>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListExportsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_summaries(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ExportSummary>>) -> Self {
    self.export_summaries = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_summaries(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ExportSummary>>) -> Self {
    self.export_summaries = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_summaries(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ExportSummary>> {
    &self.export_summaries
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
    /// Consumes the builder and constructs a [`ListExportsOutput`](crate::operation::operation::ListExportsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_exports::ListExportsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_exports::ListExportsOutput {
            export_summaries: self.export_summaries,
next_token: self.next_token,
        })
    }
}
