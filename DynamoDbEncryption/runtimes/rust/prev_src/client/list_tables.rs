// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListTables`](crate::operation::list_tables::builders::ListTablesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`exclusive_start_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::exclusive_start_table_name) / [`set_exclusive_start_table_name(Option<::std::string::String>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::set_exclusive_start_table_name): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_tables::builders::ListTablesFluentBuilder::set_limit): (undocumented)<br>
    /// - On success, responds with [`ListTablesOutput`](crate::operation::list_tables::ListTablesOutput) with field(s):
    ///   - [`last_evaluated_table_name(Option<::std::string::String>)`](crate::operation::list_tables::ListTablesOutput::last_evaluated_table_name): (undocumented)
    ///   - [`table_names(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::list_tables::ListTablesOutput::table_names): (undocumented)
    /// - On failure, responds with [`SdkError<ListTablesError>`](crate::operation::list_tables::ListTablesError)
    pub fn list_tables(&self) -> crate::operation::list_tables::builders::ListTablesFluentBuilder {
        crate::operation::list_tables::builders::ListTablesFluentBuilder::new(self.clone())
    }
}
