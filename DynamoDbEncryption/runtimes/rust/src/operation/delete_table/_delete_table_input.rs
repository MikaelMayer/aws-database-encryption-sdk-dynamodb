// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DeleteTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DeleteTableInput {
    /// Creates a new builder-style object to manufacture [`DeleteTableInput`](crate::operation::delete_table::builders::DeleteTableInput).
    pub fn builder() -> crate::operation::delete_table::builders::DeleteTableInputBuilder {
        crate::operation::delete_table::builders::DeleteTableInputBuilder::default()
    }
}

/// A builder for [`DeleteTableInput`](crate::operation::operation::DeleteTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteTableInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DeleteTableInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteTableInput`](crate::operation::operation::DeleteTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_table::DeleteTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_table::DeleteTableInput {
            table_name: self.table_name,
        })
    }
}
