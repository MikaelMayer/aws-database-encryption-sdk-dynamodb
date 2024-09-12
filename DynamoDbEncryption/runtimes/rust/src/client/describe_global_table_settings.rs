// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeGlobalTableSettings`](crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsFluentBuilder::global_table_name) / [`set_global_table_name(Option<::std::string::String>)`](crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsFluentBuilder::set_global_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeGlobalTableSettingsOutput`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput) with field(s):
    ///   - [`global_table_name(Option<::std::string::String>)`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput::global_table_name): (undocumented)
    ///   - [`replica_settings(Option<::std::vec::Vec<dynamodb::types::ReplicaSettingsDescription>>)`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput::replica_settings): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeGlobalTableSettingsError>`](crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsError)
    pub fn describe_global_table_settings(&self) -> crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsFluentBuilder {
        crate::operation::describe_global_table_settings::builders::DescribeGlobalTableSettingsFluentBuilder::new(self.clone())
    }
}
