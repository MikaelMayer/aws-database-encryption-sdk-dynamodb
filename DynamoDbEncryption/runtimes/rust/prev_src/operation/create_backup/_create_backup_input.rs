// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub backup_name: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl CreateBackupInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl CreateBackupInput {
    /// Creates a new builder-style object to manufacture [`CreateBackupInput`](crate::operation::create_backup::builders::CreateBackupInput).
    pub fn builder() -> crate::operation::create_backup::builders::CreateBackupInputBuilder {
        crate::operation::create_backup::builders::CreateBackupInputBuilder::default()
    }
}

/// A builder for [`CreateBackupInput`](crate::operation::operation::CreateBackupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateBackupInputBuilder {
    pub(crate) backup_name: ::std::option::Option<::std::string::String>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl CreateBackupInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn backup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.backup_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_backup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.backup_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_backup_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.backup_name
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`CreateBackupInput`](crate::operation::operation::CreateBackupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_backup::CreateBackupInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_backup::CreateBackupInput {
            backup_name: self.backup_name,
table_name: self.table_name,
        })
    }
}
