// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGlobalTableSettingsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub replica_settings: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>>,
}
impl DescribeGlobalTableSettingsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn replica_settings(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>> {
    &self.replica_settings
}
}
impl DescribeGlobalTableSettingsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableSettingsOutput`](crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsOutput).
    pub fn builder() -> crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsOutputBuilder {
        crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsOutputBuilder::default()
    }
}

/// A builder for [`DescribeGlobalTableSettingsOutput`](crate::operation::operation::DescribeGlobalTableSettingsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGlobalTableSettingsOutputBuilder {
    pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
pub(crate) replica_settings: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>>,
}
impl DescribeGlobalTableSettingsOutputBuilder {
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
#[allow(missing_docs)] // documentation missing in model
pub fn replica_settings(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>>) -> Self {
    self.replica_settings = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replica_settings(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>>) -> Self {
    self.replica_settings = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replica_settings(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>> {
    &self.replica_settings
}
    /// Consumes the builder and constructs a [`DescribeGlobalTableSettingsOutput`](crate::operation::operation::DescribeGlobalTableSettingsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput {
            global_table_name: self.global_table_name,
replica_settings: self.replica_settings,
        })
    }
}
