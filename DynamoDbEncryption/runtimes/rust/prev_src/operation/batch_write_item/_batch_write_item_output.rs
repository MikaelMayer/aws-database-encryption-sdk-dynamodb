// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchWriteItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
#[allow(missing_docs)] // documentation missing in model
pub item_collection_metrics: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>,
#[allow(missing_docs)] // documentation missing in model
pub unprocessed_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>,
}
impl BatchWriteItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item_collection_metrics(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>> {
    &self.item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn unprocessed_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>> {
    &self.unprocessed_items
}
}
impl BatchWriteItemOutput {
    /// Creates a new builder-style object to manufacture [`BatchWriteItemOutput`](crate::operation::batch_write_item::builders::BatchWriteItemOutput).
    pub fn builder() -> crate::operation::batch_write_item::builders::BatchWriteItemOutputBuilder {
        crate::operation::batch_write_item::builders::BatchWriteItemOutputBuilder::default()
    }
}

/// A builder for [`BatchWriteItemOutput`](crate::operation::operation::BatchWriteItemOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchWriteItemOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
pub(crate) item_collection_metrics: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>,
pub(crate) unprocessed_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>,
}
impl BatchWriteItemOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consumed_capacity(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item_collection_metrics(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>) -> Self {
    self.item_collection_metrics = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item_collection_metrics(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>) -> Self {
    self.item_collection_metrics = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item_collection_metrics(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>> {
    &self.item_collection_metrics
}
#[allow(missing_docs)] // documentation missing in model
pub fn unprocessed_items(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>) -> Self {
    self.unprocessed_items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_unprocessed_items(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>>) -> Self {
    self.unprocessed_items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_unprocessed_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::WriteRequest>>> {
    &self.unprocessed_items
}
    /// Consumes the builder and constructs a [`BatchWriteItemOutput`](crate::operation::operation::BatchWriteItemOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_write_item::BatchWriteItemOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_write_item::BatchWriteItemOutput {
            consumed_capacity: self.consumed_capacity,
item_collection_metrics: self.item_collection_metrics,
unprocessed_items: self.unprocessed_items,
        })
    }
}
