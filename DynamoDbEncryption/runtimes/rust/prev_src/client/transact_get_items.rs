// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`TransactGetItems`](crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`transact_items(impl Into<Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>>)`](crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder::transact_items) / [`set_transact_items(Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>)`](crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder::set_transact_items): (undocumented)<br>
    /// - On success, responds with [`TransactGetItemsOutput`](crate::operation::transact_get_items::TransactGetItemsOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::transact_get_items::TransactGetItemsOutput::consumed_capacity): (undocumented)
    ///   - [`responses(Option<::std::vec::Vec<dynamodb::types::ItemResponse>>)`](crate::operation::transact_get_items::TransactGetItemsOutput::responses): (undocumented)
    /// - On failure, responds with [`SdkError<TransactGetItemsError>`](crate::operation::transact_get_items::TransactGetItemsError)
    pub fn transact_get_items(&self) -> crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder {
        crate::operation::transact_get_items::builders::TransactGetItemsFluentBuilder::new(self.clone())
    }
}
