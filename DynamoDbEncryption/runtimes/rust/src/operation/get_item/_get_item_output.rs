// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
}
impl GetItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.item
}
}
impl GetItemOutput {
    /// Creates a new builder-style object to manufacture [`GetItemOutput`](crate::operation::get_item::builders::GetItemOutput).
    pub fn builder() -> crate::operation::get_item::builders::GetItemOutputBuilder {
        crate::operation::get_item::builders::GetItemOutputBuilder::default()
    }
}

/// A builder for [`GetItemOutput`](crate::operation::operation::GetItemOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetItemOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
pub(crate) item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
}
impl GetItemOutputBuilder {
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
pub fn item(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.item = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.item = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.item
}
    /// Consumes the builder and constructs a [`GetItemOutput`](crate::operation::operation::GetItemOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_item::GetItemOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_item::GetItemOutput {
            consumed_capacity: self.consumed_capacity,
item: self.item,
        })
    }
}
