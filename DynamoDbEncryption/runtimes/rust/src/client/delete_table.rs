// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeleteTable`](crate::operation::delete_table::builders::DeleteTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::delete_table::builders::DeleteTableFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DeleteTableOutput`](crate::operation::delete_table::DeleteTableOutput) with field(s):
    ///   - [`table_description(Option<dynamodb::types::TableDescription>)`](crate::operation::delete_table::DeleteTableOutput::table_description): (undocumented)
    /// - On failure, responds with [`SdkError<DeleteTableError>`](crate::operation::delete_table::DeleteTableError)
    pub fn delete_table(&self) -> crate::operation::delete_table::builders::DeleteTableFluentBuilder {
        crate::operation::delete_table::builders::DeleteTableFluentBuilder::new(self.clone())
    }
}
