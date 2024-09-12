// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactWriteItemsInput {
    #[allow(missing_docs)] // documentation missing in model
pub client_request_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
#[allow(missing_docs)] // documentation missing in model
pub transact_items: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>,
}
impl TransactWriteItemsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_request_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_request_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    &self.return_item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>> {
    &self.transact_items
}
}
impl TransactWriteItemsInput {
    /// Creates a new builder-style object to manufacture [`TransactWriteItemsInput`](crate::operation::transact_write_items::builders::TransactWriteItemsInput).
    pub fn builder() -> crate::operation::transact_write_items::builders::TransactWriteItemsInputBuilder {
        crate::operation::transact_write_items::builders::TransactWriteItemsInputBuilder::default()
    }
}

/// A builder for [`TransactWriteItemsInput`](crate::operation::operation::TransactWriteItemsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransactWriteItemsInputBuilder {
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
pub(crate) transact_items: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>,
}
impl TransactWriteItemsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.client_request_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.client_request_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_request_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_item_collection_metrics(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.return_item_collection_metrics = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>) -> Self {
    self.return_item_collection_metrics = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    &self.return_item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::TransactWriteItem>>) -> Self {
    self.transact_items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transact_items(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>>) -> Self {
    self.transact_items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactWriteItem>> {
    &self.transact_items
}
    /// Consumes the builder and constructs a [`TransactWriteItemsInput`](crate::operation::operation::TransactWriteItemsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_write_items::TransactWriteItemsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::transact_write_items::TransactWriteItemsInput {
            client_request_token: self.client_request_token,
return_consumed_capacity: self.return_consumed_capacity,
return_item_collection_metrics: self.return_item_collection_metrics,
transact_items: self.transact_items,
        })
    }
}
