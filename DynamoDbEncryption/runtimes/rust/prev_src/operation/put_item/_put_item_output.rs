// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub item_collection_metrics: ::std::option::Option<dynamodb::types::ItemCollectionMetrics>,
}
impl PutItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.attributes
}
#[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ItemCollectionMetrics> {
    &self.item_collection_metrics
}
}
impl PutItemOutput {
    /// Creates a new builder-style object to manufacture [`PutItemOutput`](crate::operation::put_item::builders::PutItemOutput).
    pub fn builder() -> crate::operation::put_item::builders::PutItemOutputBuilder {
        crate::operation::put_item::builders::PutItemOutputBuilder::default()
    }
}

/// A builder for [`PutItemOutput`](crate::operation::operation::PutItemOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutItemOutputBuilder {
    pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
pub(crate) item_collection_metrics: ::std::option::Option<dynamodb::types::ItemCollectionMetrics>,
}
impl PutItemOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn attributes(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.attributes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.attributes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.attributes
}
#[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ConsumedCapacity>) -> Self {
    self.consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ConsumedCapacity>) -> Self {
    self.consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item_collection_metrics(mut self, input: impl ::std::convert::Into<dynamodb::types::ItemCollectionMetrics>) -> Self {
    self.item_collection_metrics = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item_collection_metrics(mut self, input: ::std::option::Option<dynamodb::types::ItemCollectionMetrics>) -> Self {
    self.item_collection_metrics = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item_collection_metrics(&self) -> &::std::option::Option<dynamodb::types::ItemCollectionMetrics> {
    &self.item_collection_metrics
}
    /// Consumes the builder and constructs a [`PutItemOutput`](crate::operation::operation::PutItemOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_item::PutItemOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_item::PutItemOutput {
            attributes: self.attributes,
consumed_capacity: self.consumed_capacity,
item_collection_metrics: self.item_collection_metrics,
        })
    }
}