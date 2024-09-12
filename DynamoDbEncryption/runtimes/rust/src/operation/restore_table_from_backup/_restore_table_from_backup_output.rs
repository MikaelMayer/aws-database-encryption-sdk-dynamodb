// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreTableFromBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl RestoreTableFromBackupOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_description(&self) -> &::std::option::Option<dynamodb::types::TableDescription> {
    &self.table_description
}
}
impl RestoreTableFromBackupOutput {
    /// Creates a new builder-style object to manufacture [`RestoreTableFromBackupOutput`](crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupOutput).
    pub fn builder() -> crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupOutputBuilder {
        crate::operation::restore_table_from_backup::builders::RestoreTableFromBackupOutputBuilder::default()
    }
}

/// A builder for [`RestoreTableFromBackupOutput`](crate::operation::operation::RestoreTableFromBackupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreTableFromBackupOutputBuilder {
    pub(crate) table_description: ::std::option::Option<dynamodb::types::TableDescription>,
}
impl RestoreTableFromBackupOutputBuilder {
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
    /// Consumes the builder and constructs a [`RestoreTableFromBackupOutput`](crate::operation::operation::RestoreTableFromBackupOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::restore_table_from_backup::RestoreTableFromBackupOutput {
            table_description: self.table_description,
        })
    }
}
