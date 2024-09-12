// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl DeleteTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_description(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table_description
}
}
impl DeleteTableOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTableOutput`](crate::operation::delete_table::builders::DeleteTableOutput).
    pub fn builder() -> crate::operation::delete_table::builders::DeleteTableOutputBuilder {
        crate::operation::delete_table::builders::DeleteTableOutputBuilder::default()
    }
}

/// A builder for [`DeleteTableOutput`](crate::operation::operation::DeleteTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteTableOutputBuilder {
    pub(crate) table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl DeleteTableOutputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteTableOutput`](crate::operation::operation::DeleteTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_table::DeleteTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_table::DeleteTableOutput {
            table_description: self.table_description,
        })
    }
}
