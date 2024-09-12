// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateGlobalTable`](crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder::global_table_name) / [`set_global_table_name(Option<::std::string::String>)`](crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder::set_global_table_name): (undocumented)<br>
    ///   - [`replica_updates(impl Into<Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>>)`](crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder::replica_updates) / [`set_replica_updates(Option<::std::vec::Vec<dynamodb::types::ReplicaUpdate>>)`](crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder::set_replica_updates): (undocumented)<br>
    /// - On success, responds with [`UpdateGlobalTableOutput`](crate::operation::update_global_table::UpdateGlobalTableOutput) with field(s):
    ///   - [`global_table_description(Option<dynamodb::types::GlobalTableDescription>)`](crate::operation::update_global_table::UpdateGlobalTableOutput::global_table_description): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateGlobalTableError>`](crate::operation::update_global_table::UpdateGlobalTableError)
    pub fn update_global_table(&self) -> crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder {
        crate::operation::update_global_table::builders::UpdateGlobalTableFluentBuilder::new(self.clone())
    }
}
