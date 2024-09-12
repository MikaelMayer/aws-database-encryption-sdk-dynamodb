// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`PutItem`](crate::operation::put_item::builders::PutItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`condition_expression(impl Into<Option<::std::string::String>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::condition_expression) / [`set_condition_expression(Option<::std::string::String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_condition_expression): (undocumented)<br>
    ///   - [`conditional_operator(impl Into<Option<dynamodb::types::ConditionalOperator>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::conditional_operator) / [`set_conditional_operator(Option<dynamodb::types::ConditionalOperator>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_conditional_operator): (undocumented)<br>
    ///   - [`expected(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::expected) / [`set_expected(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::ExpectedAttributeValue>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expected): (undocumented)<br>
    ///   - [`expression_attribute_names(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expression_attribute_names): (undocumented)<br>
    ///   - [`expression_attribute_values(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::expression_attribute_values) / [`set_expression_attribute_values(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_expression_attribute_values): (undocumented)<br>
    ///   - [`item(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::item) / [`set_item(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_item): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`return_item_collection_metrics(impl Into<Option<dynamodb::types::ReturnItemCollectionMetrics>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::return_item_collection_metrics) / [`set_return_item_collection_metrics(Option<dynamodb::types::ReturnItemCollectionMetrics>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_return_item_collection_metrics): (undocumented)<br>
    ///   - [`return_values(impl Into<Option<dynamodb::types::ReturnValue>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::return_values) / [`set_return_values(Option<dynamodb::types::ReturnValue>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_return_values): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::put_item::builders::PutItemFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::put_item::builders::PutItemFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`PutItemOutput`](crate::operation::put_item::PutItemOutput) with field(s):
    ///   - [`attributes(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::put_item::PutItemOutput::attributes): (undocumented)
    ///   - [`consumed_capacity(Option<dynamodb::types::ConsumedCapacity>)`](crate::operation::put_item::PutItemOutput::consumed_capacity): (undocumented)
    ///   - [`item_collection_metrics(Option<dynamodb::types::ItemCollectionMetrics>)`](crate::operation::put_item::PutItemOutput::item_collection_metrics): (undocumented)
    /// - On failure, responds with [`SdkError<PutItemError>`](crate::operation::put_item::PutItemError)
    pub fn put_item(&self) -> crate::operation::put_item::builders::PutItemFluentBuilder {
        crate::operation::put_item::builders::PutItemFluentBuilder::new(self.clone())
    }
}
