// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListGlobalTables`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`exclusive_start_global_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::exclusive_start_global_table_name) / [`set_exclusive_start_global_table_name(Option<::std::string::String>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::set_exclusive_start_global_table_name): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`region_name(impl Into<Option<::std::string::String>>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::region_name) / [`set_region_name(Option<::std::string::String>)`](crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::set_region_name): (undocumented)<br>
    /// - On success, responds with [`ListGlobalTablesOutput`](crate::operation::list_global_tables::ListGlobalTablesOutput) with field(s):
    ///   - [`global_tables(Option<::std::vec::Vec<dynamodb::types::GlobalTable>>)`](crate::operation::list_global_tables::ListGlobalTablesOutput::global_tables): (undocumented)
    ///   - [`last_evaluated_global_table_name(Option<::std::string::String>)`](crate::operation::list_global_tables::ListGlobalTablesOutput::last_evaluated_global_table_name): (undocumented)
    /// - On failure, responds with [`SdkError<ListGlobalTablesError>`](crate::operation::list_global_tables::ListGlobalTablesError)
    pub fn list_global_tables(&self) -> crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder {
        crate::operation::list_global_tables::builders::ListGlobalTablesFluentBuilder::new(self.clone())
    }
}
