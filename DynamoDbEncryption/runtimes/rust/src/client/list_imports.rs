// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListImports`](crate::operation::list_imports::builders::ListImportsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<Option<::std::string::String>>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::next_token) / [`set_next_token(Option<::std::string::String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_next_token): (undocumented)<br>
    ///   - [`page_size(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::page_size) / [`set_page_size(Option<::std::primitive::i32>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_page_size): (undocumented)<br>
    ///   - [`table_arn(impl Into<Option<::std::string::String>>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::table_arn) / [`set_table_arn(Option<::std::string::String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_table_arn): (undocumented)<br>
    /// - On success, responds with [`ListImportsOutput`](crate::operation::list_imports::ListImportsOutput) with field(s):
    ///   - [`import_summary_list(Option<::std::vec::Vec<dynamodb::types::ImportSummary>>)`](crate::operation::list_imports::ListImportsOutput::import_summary_list): (undocumented)
    ///   - [`next_token(Option<::std::string::String>)`](crate::operation::list_imports::ListImportsOutput::next_token): (undocumented)
    /// - On failure, responds with [`SdkError<ListImportsError>`](crate::operation::list_imports::ListImportsError)
    pub fn list_imports(&self) -> crate::operation::list_imports::builders::ListImportsFluentBuilder {
        crate::operation::list_imports::builders::ListImportsFluentBuilder::new(self.clone())
    }
}
