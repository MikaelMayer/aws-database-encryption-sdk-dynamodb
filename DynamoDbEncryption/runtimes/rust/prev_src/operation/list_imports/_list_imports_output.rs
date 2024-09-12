// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListImportsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub import_summary_list: ::std::option::Option<::std::vec::Vec<dynamodb::types::ImportSummary>>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListImportsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_summary_list(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ImportSummary>> {
    &self.import_summary_list
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
}
impl ListImportsOutput {
    /// Creates a new builder-style object to manufacture [`ListImportsOutput`](crate::operation::list_imports::builders::ListImportsOutput).
    pub fn builder() -> crate::operation::list_imports::builders::ListImportsOutputBuilder {
        crate::operation::list_imports::builders::ListImportsOutputBuilder::default()
    }
}

/// A builder for [`ListImportsOutput`](crate::operation::operation::ListImportsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListImportsOutputBuilder {
    pub(crate) import_summary_list: ::std::option::Option<::std::vec::Vec<dynamodb::types::ImportSummary>>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListImportsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_summary_list(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ImportSummary>>) -> Self {
    self.import_summary_list = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_summary_list(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ImportSummary>>) -> Self {
    self.import_summary_list = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_summary_list(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ImportSummary>> {
    &self.import_summary_list
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
    /// Consumes the builder and constructs a [`ListImportsOutput`](crate::operation::operation::ListImportsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_imports::ListImportsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_imports::ListImportsOutput {
            import_summary_list: self.import_summary_list,
next_token: self.next_token,
        })
    }
}
