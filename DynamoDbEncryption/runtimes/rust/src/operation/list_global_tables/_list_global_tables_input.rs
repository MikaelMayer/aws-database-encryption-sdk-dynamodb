// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListGlobalTablesInput {
    #[allow(missing_docs)] // documentation missing in model
pub exclusive_start_global_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub limit: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub region_name: ::std::option::Option<::std::string::String>,
}
impl ListGlobalTablesInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.exclusive_start_global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn region_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.region_name
}
}
impl ListGlobalTablesInput {
    /// Creates a new builder-style object to manufacture [`ListGlobalTablesInput`](crate::operation::list_global_tables::builders::ListGlobalTablesInput).
    pub fn builder() -> crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder {
        crate::operation::list_global_tables::builders::ListGlobalTablesInputBuilder::default()
    }
}

/// A builder for [`ListGlobalTablesInput`](crate::operation::operation::ListGlobalTablesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListGlobalTablesInputBuilder {
    pub(crate) exclusive_start_global_table_name: ::std::option::Option<::std::string::String>,
pub(crate) limit: ::std::option::Option<::std::primitive::i32>,
pub(crate) region_name: ::std::option::Option<::std::string::String>,
}
impl ListGlobalTablesInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn exclusive_start_global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.exclusive_start_global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_exclusive_start_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.exclusive_start_global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_exclusive_start_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.exclusive_start_global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn limit(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.limit = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_limit(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.limit = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_limit(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.limit
}
#[allow(missing_docs)] // documentation missing in model
pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.region_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.region_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.region_name
}
    /// Consumes the builder and constructs a [`ListGlobalTablesInput`](crate::operation::operation::ListGlobalTablesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_global_tables::ListGlobalTablesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_global_tables::ListGlobalTablesInput {
            exclusive_start_global_table_name: self.exclusive_start_global_table_name,
limit: self.limit,
region_name: self.region_name,
        })
    }
}
