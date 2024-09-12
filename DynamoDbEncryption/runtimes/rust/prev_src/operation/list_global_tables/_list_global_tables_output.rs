// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListGlobalTablesOutput {
    #[allow(missing_docs)] // documentation missing in model
pub global_tables: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTable>>,
#[allow(missing_docs)] // documentation missing in model
pub last_evaluated_global_table_name: ::std::option::Option<::std::string::String>,
}
impl ListGlobalTablesOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_tables(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTable>> {
    &self.global_tables
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_global_table_name
}
}
impl ListGlobalTablesOutput {
    /// Creates a new builder-style object to manufacture [`ListGlobalTablesOutput`](crate::operation::list_global_tables::builders::ListGlobalTablesOutput).
    pub fn builder() -> crate::operation::list_global_tables::builders::ListGlobalTablesOutputBuilder {
        crate::operation::list_global_tables::builders::ListGlobalTablesOutputBuilder::default()
    }
}

/// A builder for [`ListGlobalTablesOutput`](crate::operation::operation::ListGlobalTablesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListGlobalTablesOutputBuilder {
    pub(crate) global_tables: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTable>>,
pub(crate) last_evaluated_global_table_name: ::std::option::Option<::std::string::String>,
}
impl ListGlobalTablesOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_tables(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::GlobalTable>>) -> Self {
    self.global_tables = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_tables(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTable>>) -> Self {
    self.global_tables = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_tables(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::GlobalTable>> {
    &self.global_tables
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.last_evaluated_global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_evaluated_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.last_evaluated_global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_evaluated_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.last_evaluated_global_table_name
}
    /// Consumes the builder and constructs a [`ListGlobalTablesOutput`](crate::operation::operation::ListGlobalTablesOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_global_tables::ListGlobalTablesOutput {
            global_tables: self.global_tables,
last_evaluated_global_table_name: self.last_evaluated_global_table_name,
        })
    }
}
