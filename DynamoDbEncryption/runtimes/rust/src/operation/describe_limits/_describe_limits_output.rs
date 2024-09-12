// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub account_max_read_capacity_units: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)] // documentation missing in model
pub account_max_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)] // documentation missing in model
pub table_max_read_capacity_units: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)] // documentation missing in model
pub table_max_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
}
impl DescribeLimitsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn account_max_read_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.account_max_read_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn account_max_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.account_max_write_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_max_read_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.table_max_read_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_max_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.table_max_write_capacity_units
}
}
impl DescribeLimitsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLimitsOutput`](crate::operation::describe_limits::builders::DescribeLimitsOutput).
    pub fn builder() -> crate::operation::describe_limits::builders::DescribeLimitsOutputBuilder {
        crate::operation::describe_limits::builders::DescribeLimitsOutputBuilder::default()
    }
}

/// A builder for [`DescribeLimitsOutput`](crate::operation::operation::DescribeLimitsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeLimitsOutputBuilder {
    pub(crate) account_max_read_capacity_units: ::std::option::Option<::std::primitive::i64>,
pub(crate) account_max_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
pub(crate) table_max_read_capacity_units: ::std::option::Option<::std::primitive::i64>,
pub(crate) table_max_write_capacity_units: ::std::option::Option<::std::primitive::i64>,
}
impl DescribeLimitsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn account_max_read_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.account_max_read_capacity_units = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_account_max_read_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.account_max_read_capacity_units = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_account_max_read_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.account_max_read_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn account_max_write_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.account_max_write_capacity_units = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_account_max_write_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.account_max_write_capacity_units = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_account_max_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.account_max_write_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_max_read_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.table_max_read_capacity_units = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_max_read_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.table_max_read_capacity_units = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_max_read_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.table_max_read_capacity_units
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_max_write_capacity_units(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.table_max_write_capacity_units = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_max_write_capacity_units(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.table_max_write_capacity_units = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_max_write_capacity_units(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.table_max_write_capacity_units
}
    /// Consumes the builder and constructs a [`DescribeLimitsOutput`](crate::operation::operation::DescribeLimitsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_limits::DescribeLimitsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_limits::DescribeLimitsOutput {
            account_max_read_capacity_units: self.account_max_read_capacity_units,
account_max_write_capacity_units: self.account_max_write_capacity_units,
table_max_read_capacity_units: self.table_max_read_capacity_units,
table_max_write_capacity_units: self.table_max_write_capacity_units,
        })
    }
}
