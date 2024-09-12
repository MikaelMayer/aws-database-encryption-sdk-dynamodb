// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetItem`](crate::operation::get_item::builders::GetItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attributes_to_get(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::attributes_to_get) / [`set_attributes_to_get(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_attributes_to_get): (undocumented)<br>
    ///   - [`consistent_read(impl Into<Option<::std::primitive::bool>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::consistent_read) / [`set_consistent_read(Option<::std::primitive::bool>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_consistent_read): (undocumented)<br>
    ///   - [`expression_attribute_names(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::expression_attribute_names) / [`set_expression_attribute_names(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_expression_attribute_names): (undocumented)<br>
    ///   - [`key(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::key) / [`set_key(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_key): (undocumented)<br>
    ///   - [`projection_expression(impl Into<Option<::std::string::String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::projection_expression) / [`set_projection_expression(Option<::std::string::String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_projection_expression): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::get_item::builders::GetItemFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::get_item::builders::GetItemFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`GetItemOutput`](crate::operation::get_item::GetItemOutput) with field(s):
    ///   - [`consumed_capacity(Option<dynamodb::types::ConsumedCapacity>)`](crate::operation::get_item::GetItemOutput::consumed_capacity): (undocumented)
    ///   - [`item(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::get_item::GetItemOutput::item): (undocumented)
    /// - On failure, responds with [`SdkError<GetItemError>`](crate::operation::get_item::GetItemError)
    pub fn get_item(&self) -> crate::operation::get_item::builders::GetItemFluentBuilder {
        crate::operation::get_item::builders::GetItemFluentBuilder::new(self.clone())
    }
}
