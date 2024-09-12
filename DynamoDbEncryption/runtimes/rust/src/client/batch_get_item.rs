// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`BatchGetItem`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`request_items(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::request_items) / [`set_request_items(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_request_items): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    /// - On success, responds with [`BatchGetItemOutput`](crate::operation::batch_get_item::BatchGetItemOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::batch_get_item::BatchGetItemOutput::consumed_capacity): (undocumented)
    ///   - [`responses(Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>>)`](crate::operation::batch_get_item::BatchGetItemOutput::responses): (undocumented)
    ///   - [`unprocessed_keys(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>)`](crate::operation::batch_get_item::BatchGetItemOutput::unprocessed_keys): (undocumented)
    /// - On failure, responds with [`SdkError<BatchGetItemError>`](crate::operation::batch_get_item::BatchGetItemError)
    pub fn batch_get_item(&self) -> crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder {
        crate::operation::batch_get_item::builders::BatchGetItemFluentBuilder::new(self.clone())
    }
}
