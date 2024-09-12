// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScanOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub count: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>,
#[allow(missing_docs)] // documentation missing in model
pub last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub scanned_count: ::std::option::Option<::std::primitive::i32>,
}
impl ScanOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ConsumedCapacity> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn count(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.count
}
#[allow(missing_docs)] // documentation missing in model
pub fn items(&self) -> &::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>> {
    &self.items
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.last_evaluated_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn scanned_count(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.scanned_count
}
}
impl ScanOutput {
    /// Creates a new builder-style object to manufacture [`ScanOutput`](crate::operation::scan::builders::ScanOutput).
    pub fn builder() -> crate::operation::scan::builders::ScanOutputBuilder {
        crate::operation::scan::builders::ScanOutputBuilder::default()
    }
}

/// A builder for [`ScanOutput`](crate::operation::operation::ScanOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScanOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<dynamodb::types::ConsumedCapacity>,
pub(crate) count: ::std::option::Option<::std::primitive::i32>,
pub(crate) items: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>,
pub(crate) last_evaluated_key: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) scanned_count: ::std::option::Option<::std::primitive::i32>,
}
impl ScanOutputBuilder {
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
pub fn count(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.count = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_count(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.count = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_count(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.count
}
#[allow(missing_docs)] // documentation missing in model
pub fn items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>) -> Self {
    self.items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_items(mut self, input: ::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>) -> Self {
    self.items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_items(&self) -> &::std::option::Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>> {
    &self.items
}
#[allow(missing_docs)] // documentation missing in model
pub fn last_evaluated_key(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.last_evaluated_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_last_evaluated_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.last_evaluated_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_last_evaluated_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.last_evaluated_key
}
#[allow(missing_docs)] // documentation missing in model
pub fn scanned_count(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.scanned_count = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_scanned_count(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.scanned_count = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_scanned_count(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.scanned_count
}
    /// Consumes the builder and constructs a [`ScanOutput`](crate::operation::operation::ScanOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::scan::ScanOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::scan::ScanOutput {
            consumed_capacity: self.consumed_capacity,
count: self.count,
items: self.items,
last_evaluated_key: self.last_evaluated_key,
scanned_count: self.scanned_count,
        })
    }
}
