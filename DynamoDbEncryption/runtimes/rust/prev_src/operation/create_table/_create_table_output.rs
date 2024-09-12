// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl CreateTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_description(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table_description
}
}
impl CreateTableOutput {
    /// Creates a new builder-style object to manufacture [`CreateTableOutput`](crate::operation::create_table::builders::CreateTableOutput).
    pub fn builder() -> crate::operation::create_table::builders::CreateTableOutputBuilder {
        crate::operation::create_table::builders::CreateTableOutputBuilder::default()
    }
}

/// A builder for [`CreateTableOutput`](crate::operation::operation::CreateTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateTableOutputBuilder {
    pub(crate) table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl CreateTableOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_description(mut self, input: impl ::std::convert::Into<dynamodb::types::TableDescription>) -> Self {
    self.table_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_description(mut self, input: ::std::option::Option<dynamodb::types::TableDescription>) -> Self {
    self.table_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_description(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table_description
}
    /// Consumes the builder and constructs a [`CreateTableOutput`](crate::operation::operation::CreateTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_table::CreateTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_table::CreateTableOutput {
            table_description: self.table_description,
        })
    }
}
