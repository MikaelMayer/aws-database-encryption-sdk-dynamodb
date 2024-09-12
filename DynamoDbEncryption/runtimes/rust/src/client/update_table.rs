// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateTable`](crate::operation::update_table::builders::UpdateTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attribute_definitions(impl Into<Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::attribute_definitions) / [`set_attribute_definitions(Option<::std::vec::Vec<dynamodb::types::AttributeDefinition>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_attribute_definitions): (undocumented)<br>
    ///   - [`billing_mode(impl Into<Option<dynamodb::types::BillingMode>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::billing_mode) / [`set_billing_mode(Option<dynamodb::types::BillingMode>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_billing_mode): (undocumented)<br>
    ///   - [`global_secondary_index_updates(impl Into<Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::global_secondary_index_updates) / [`set_global_secondary_index_updates(Option<::std::vec::Vec<dynamodb::types::GlobalSecondaryIndexUpdate>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_global_secondary_index_updates): (undocumented)<br>
    ///   - [`provisioned_throughput(impl Into<Option<dynamodb::types::ProvisionedThroughput>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::provisioned_throughput) / [`set_provisioned_throughput(Option<dynamodb::types::ProvisionedThroughput>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_provisioned_throughput): (undocumented)<br>
    ///   - [`replica_updates(impl Into<Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::replica_updates) / [`set_replica_updates(Option<::std::vec::Vec<dynamodb::types::ReplicationGroupUpdate>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_replica_updates): (undocumented)<br>
    ///   - [`sse_specification(impl Into<Option<dynamodb::types::SseSpecification>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::sse_specification) / [`set_sse_specification(Option<dynamodb::types::SseSpecification>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_sse_specification): (undocumented)<br>
    ///   - [`stream_specification(impl Into<Option<dynamodb::types::StreamSpecification>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::stream_specification) / [`set_stream_specification(Option<dynamodb::types::StreamSpecification>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_stream_specification): (undocumented)<br>
    ///   - [`table_class(impl Into<Option<dynamodb::types::TableClass>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::table_class) / [`set_table_class(Option<dynamodb::types::TableClass>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_table_class): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::update_table::builders::UpdateTableFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`UpdateTableOutput`](crate::operation::update_table::UpdateTableOutput) with field(s):
    ///   - [`table_description(Option<dynamodb::types::TableDescription>)`](crate::operation::update_table::UpdateTableOutput::table_description): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateTableError>`](crate::operation::update_table::UpdateTableError)
    pub fn update_table(&self) -> crate::operation::update_table::builders::UpdateTableFluentBuilder {
        crate::operation::update_table::builders::UpdateTableFluentBuilder::new(self.clone())
    }
}
