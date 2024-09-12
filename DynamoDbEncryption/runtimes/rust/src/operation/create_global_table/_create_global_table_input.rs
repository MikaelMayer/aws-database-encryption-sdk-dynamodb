// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub replication_group: ::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>>,
}
impl CreateGlobalTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn replication_group(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>> {
    &self.replication_group
}
}
impl CreateGlobalTableInput {
    /// Creates a new builder-style object to manufacture [`CreateGlobalTableInput`](crate::operation::create_global_table::builders::CreateGlobalTableInput).
    pub fn builder() -> crate::operation::create_global_table::builders::CreateGlobalTableInputBuilder {
        crate::operation::create_global_table::builders::CreateGlobalTableInputBuilder::default()
    }
}

/// A builder for [`CreateGlobalTableInput`](crate::operation::operation::CreateGlobalTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateGlobalTableInputBuilder {
    pub(crate) global_table_name: ::std::option::Option<::std::string::String>,
pub(crate) replication_group: ::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>>,
}
impl CreateGlobalTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.global_table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.global_table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.global_table_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn replication_group(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Replica>>) -> Self {
    self.replication_group = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_replication_group(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>>) -> Self {
    self.replication_group = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_replication_group(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Replica>> {
    &self.replication_group
}
    /// Consumes the builder and constructs a [`CreateGlobalTableInput`](crate::operation::operation::CreateGlobalTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_global_table::CreateGlobalTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_global_table::CreateGlobalTableInput {
            global_table_name: self.global_table_name,
replication_group: self.replication_group,
        })
    }
}
