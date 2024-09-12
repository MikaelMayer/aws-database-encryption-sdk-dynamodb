// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchWriteItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
}
impl BatchWriteItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>> {
    &self.request_items
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics> {
    &self.return_item_collection_metrics
}
}
impl BatchWriteItemInput {
    /// Creates a new builder-style object to manufacture [`BatchWriteItemInput`](crate::operation::batch_write_item::builders::BatchWriteItemInput).
    pub fn builder() -> crate::operation::batch_write_item::builders::BatchWriteItemInputBuilder {
        crate::operation::batch_write_item::builders::BatchWriteItemInputBuilder::default()
    }
}

/// A builder for [`BatchWriteItemInput`](crate::operation::operation::BatchWriteItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchWriteItemInputBuilder {
    pub(crate) request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) return_item_collection_metrics: ::std::option::Option<dynamodb::types::ReturnItemCollectionMetrics>,
}
impl BatchWriteItemInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn request_items(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>) -> Self {
    self.request_items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_request_items(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>) -> Self {
    self.request_items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>> {
    &self.request_items
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
    /// Consumes the builder and constructs a [`BatchWriteItemInput`](crate::operation::operation::BatchWriteItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_write_item::BatchWriteItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_write_item::BatchWriteItemInput {
            request_items: self.request_items,
return_consumed_capacity: self.return_consumed_capacity,
return_item_collection_metrics: self.return_item_collection_metrics,
        })
    }
}
