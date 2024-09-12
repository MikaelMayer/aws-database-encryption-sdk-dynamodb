// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`BatchWriteItem`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`request_items(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::request_items) / [`set_request_items(Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::set_request_items): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`return_item_collection_metrics(impl Into<Option<dynamodb::types::ReturnItemCollectionMetrics>>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<dynamodb::types::ReturnItemCollectionMetrics>)`](crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::set_return_item_collection_metrics): (undocumented)<br>
    /// - On success, responds with [`BatchWriteItemOutput`](crate::operation::batch_write_item::BatchWriteItemOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::batch_write_item::BatchWriteItemOutput::consumed_capacity): (undocumented)
    ///   - [`item_collection_metrics(Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>)`](crate::operation::batch_write_item::BatchWriteItemOutput::item_collection_metrics): (undocumented)
    ///   - [`unprocessed_items(Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>)`](crate::operation::batch_write_item::BatchWriteItemOutput::unprocessed_items): (undocumented)
    /// - On failure, responds with [`SdkError<BatchWriteItemError>`](crate::operation::batch_write_item::BatchWriteItemError)
    pub fn batch_write_item(&self) -> crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder {
        crate::operation::batch_write_item::builders::BatchWriteItemFluentBuilder::new(self.clone())
    }
}
