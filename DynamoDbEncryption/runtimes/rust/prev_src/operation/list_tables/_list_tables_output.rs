// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTablesOutput {
    #[allow(missing_docs)] // documentation missing in model
pub last_evaluated_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ListTablesOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.table_names
}
}
impl ListTablesOutput {
    /// Creates a new builder-style object to manufacture [`ListTablesOutput`](crate::operation::list_tables::builders::ListTablesOutput).
    pub fn builder() -> crate::operation::list_tables::builders::ListTablesOutputBuilder {
        crate::operation::list_tables::builders::ListTablesOutputBuilder::default()
    }
}

/// A builder for [`ListTablesOutput`](crate::operation::operation::ListTablesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTablesOutputBuilder {
    pub(crate) last_evaluated_table_name: ::std::option::Option<::std::string::String>,
pub(crate) table_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ListTablesOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.last_evaluated_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_evaluated_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.last_evaluated_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_evaluated_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_names(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.table_names = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.table_names = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.table_names
}
    /// Consumes the builder and constructs a [`ListTablesOutput`](crate::operation::operation::ListTablesOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tables::ListTablesOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_tables::ListTablesOutput {
            last_evaluated_table_name: self.last_evaluated_table_name,
table_names: self.table_names,
        })
    }
}
