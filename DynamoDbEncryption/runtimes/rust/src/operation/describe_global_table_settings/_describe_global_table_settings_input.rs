// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGlobalTableSettingsInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeGlobalTableSettingsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
}
impl DescribeGlobalTableSettingsInput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableSettingsInput`](crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInput).
    pub fn builder() -> crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInputBuilder {
        crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsInputBuilder::default()
    }
}

/// A builder for [`DescribeGlobalTableSettingsInput`](crate::operation::operation::DescribeGlobalTableSettingsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGlobalTableSettingsInputBuilder {
    pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
}
impl DescribeGlobalTableSettingsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
    /// Consumes the builder and constructs a [`DescribeGlobalTableSettingsInput`](crate::operation::operation::DescribeGlobalTableSettingsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput {
            global_table_name: self.global_table_name,
        })
    }
}
