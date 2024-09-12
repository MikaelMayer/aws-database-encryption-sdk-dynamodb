// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListImportsInput {
    #[allow(missing_docs)] // documentation missing in model
pub next_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub page_size: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub table_arn: ::std::option::Option<::std::string::String>,
}
impl ListImportsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn next_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.next_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn page_size(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.page_size
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_arn(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_arn
}
}
impl ListImportsInput {
    /// Creates a new builder-style object to manufacture [`ListImportsInput`](crate::operation::list_imports::builders::ListImportsInput).
    pub fn builder() -> crate::operation::list_imports::builders::ListImportsInputBuilder {
        crate::operation::list_imports::builders::ListImportsInputBuilder::default()
    }
}

/// A builder for [`ListImportsInput`](crate::operation::operation::ListImportsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListImportsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
pub(crate) page_size: ::std::option::Option<::std::primitive::i32>,
pub(crate) table_arn: ::std::option::Option<::std::string::String>,
}
impl ListImportsInputBuilder {
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
pub fn page_size(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.page_size = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_page_size(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.page_size = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_page_size(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.page_size
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
    /// Consumes the builder and constructs a [`ListImportsInput`](crate::operation::operation::ListImportsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_imports::ListImportsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_imports::ListImportsInput {
            next_token: self.next_token,
page_size: self.page_size,
table_arn: self.table_arn,
        })
    }
}
