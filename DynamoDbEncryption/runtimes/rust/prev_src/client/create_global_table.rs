// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateGlobalTable`](crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder::global_table_name) / [`set_global_table_name(Option<::std::string::String>)`](crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder::set_global_table_name): (undocumented)<br>
    ///   - [`replication_group(impl Into<Option<::std::vec::Vec<dynamodb::types::Replica>>>)`](crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder::replication_group) / [`set_replication_group(Option<::std::vec::Vec<dynamodb::types::Replica>>)`](crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder::set_replication_group): (undocumented)<br>
    /// - On success, responds with [`CreateGlobalTableOutput`](crate::operation::create_global_table::CreateGlobalTableOutput) with field(s):
    ///   - [`global_table_description(Option<dynamodb::types::GlobalTableDescription>)`](crate::operation::create_global_table::CreateGlobalTableOutput::global_table_description): (undocumented)
    /// - On failure, responds with [`SdkError<CreateGlobalTableError>`](crate::operation::create_global_table::CreateGlobalTableError)
    pub fn create_global_table(&self) -> crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder {
        crate::operation::create_global_table::builders::CreateGlobalTableFluentBuilder::new(self.clone())
    }
}
