// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreTableToPointInTimeOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl RestoreTableToPointInTimeOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_description(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table_description
}
}
impl RestoreTableToPointInTimeOutput {
    /// Creates a new builder-style object to manufacture [`RestoreTableToPointInTimeOutput`](crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeOutput).
    pub fn builder() -> crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeOutputBuilder {
        crate::operation::restore_table_to_point_in_time::builders::RestoreTableToPointInTimeOutputBuilder::default()
    }
}

/// A builder for [`RestoreTableToPointInTimeOutput`](crate::operation::operation::RestoreTableToPointInTimeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreTableToPointInTimeOutputBuilder {
    pub(crate) table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl RestoreTableToPointInTimeOutputBuilder {
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
    /// Consumes the builder and constructs a [`RestoreTableToPointInTimeOutput`](crate::operation::operation::RestoreTableToPointInTimeOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeOutput {
            table_description: self.table_description,
        })
    }
}
