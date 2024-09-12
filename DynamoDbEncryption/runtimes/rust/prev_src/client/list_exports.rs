// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListExports`](crate::operation::list_exports::builders::ListExportsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::max_results) / [`set_max_results(Option<::std::primitive::i32>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::set_max_results): (undocumented)<br>
    ///   - [`next_token(impl Into<Option<::std::string::String>>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::next_token) / [`set_next_token(Option<::std::string::String>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::set_next_token): (undocumented)<br>
    ///   - [`table_arn(impl Into<Option<::std::string::String>>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::table_arn) / [`set_table_arn(Option<::std::string::String>)`](crate::operation::list_exports::builders::ListExportsFluentBuilder::set_table_arn): (undocumented)<br>
    /// - On success, responds with [`ListExportsOutput`](crate::operation::list_exports::ListExportsOutput) with field(s):
    ///   - [`export_summaries(Option<::std::vec::Vec<dynamodb::types::ExportSummary>>)`](crate::operation::list_exports::ListExportsOutput::export_summaries): (undocumented)
    ///   - [`next_token(Option<::std::string::String>)`](crate::operation::list_exports::ListExportsOutput::next_token): (undocumented)
    /// - On failure, responds with [`SdkError<ListExportsError>`](crate::operation::list_exports::ListExportsError)
    pub fn list_exports(&self) -> crate::operation::list_exports::builders::ListExportsFluentBuilder {
        crate::operation::list_exports::builders::ListExportsFluentBuilder::new(self.clone())
    }
}
