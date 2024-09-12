// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`TransactWriteItems`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<Option<::std::string::String>>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::client_request_token) / [`set_client_request_token(Option<::std::string::String>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::set_client_request_token): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`return_item_collection_metrics(impl Into<Option<dynamodb::types::ReturnItemCollectionMetrics>>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<dynamodb::types::ReturnItemCollectionMetrics>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::set_return_item_collection_metrics): (undocumented)<br>
    ///   - [`transact_items(impl Into<Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::transact_items) / [`set_transact_items(Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>)`](crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::set_transact_items): (undocumented)<br>
    /// - On success, responds with [`TransactWriteItemsOutput`](crate::operation::transact_write_items::TransactWriteItemsOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::transact_write_items::TransactWriteItemsOutput::consumed_capacity): (undocumented)
    ///   - [`item_collection_metrics(Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>)`](crate::operation::transact_write_items::TransactWriteItemsOutput::item_collection_metrics): (undocumented)
    /// - On failure, responds with [`SdkError<TransactWriteItemsError>`](crate::operation::transact_write_items::TransactWriteItemsError)
    pub fn transact_write_items(&self) -> crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder {
        crate::operation::transact_write_items::builders::TransactWriteItemsFluentBuilder::new(self.clone())
    }
}
