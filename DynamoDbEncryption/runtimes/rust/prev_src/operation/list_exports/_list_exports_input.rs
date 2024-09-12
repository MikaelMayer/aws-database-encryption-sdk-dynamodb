// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListExportsInput {
    #[allow(missing_docs)] // documentation missing in model
pub max_results: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_arn: ::std::option::Option<::std::string::String>,
}
impl ListExportsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn max_results(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.max_results
}
#[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_arn
}
}
impl ListExportsInput {
    /// Creates a new builder-style object to manufacture [`ListExportsInput`](crate::operation::list_exports::builders::ListExportsInput).
    pub fn builder() -> crate::operation::list_exports::builders::ListExportsInputBuilder {
        crate::operation::list_exports::builders::ListExportsInputBuilder::default()
    }
}

/// A builder for [`ListExportsInput`](crate::operation::operation::ListExportsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListExportsInputBuilder {
    pub(crate) max_results: ::std::option::Option<::std::primitive::i32>,
pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) table_arn: ::std::option::Option<::std::string::String>,
}
impl ListExportsInputBuilder {
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
pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_arn = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_arn = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_arn
}
    /// Consumes the builder and constructs a [`ListExportsInput`](crate::operation::operation::ListExportsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_exports::ListExportsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_exports::ListExportsInput {
            max_results: self.max_results,
next_token: self.next_token,
table_arn: self.table_arn,
        })
    }
}
