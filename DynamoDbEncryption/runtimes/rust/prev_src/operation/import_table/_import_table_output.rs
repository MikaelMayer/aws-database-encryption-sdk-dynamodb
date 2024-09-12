// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub import_table_description: ::std::option::Option<dynamodb::types::ImportTableDescription>,
}
impl ImportTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_table_description(&self) -> &::std::option::Option<dynamodb::types::ImportTableDescription> {
    &self.import_table_description
}
}
impl ImportTableOutput {
    /// Creates a new builder-style object to manufacture [`ImportTableOutput`](crate::operation::import_table::builders::ImportTableOutput).
    pub fn builder() -> crate::operation::import_table::builders::ImportTableOutputBuilder {
        crate::operation::import_table::builders::ImportTableOutputBuilder::default()
    }
}

/// A builder for [`ImportTableOutput`](crate::operation::operation::ImportTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportTableOutputBuilder {
    pub(crate) import_table_description: ::std::option::Option<dynamodb::types::ImportTableDescription>,
}
impl ImportTableOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_table_description(mut self, input: impl ::std::convert::Into<dynamodb::types::ImportTableDescription>) -> Self {
    self.import_table_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_table_description(mut self, input: ::std::option::Option<dynamodb::types::ImportTableDescription>) -> Self {
    self.import_table_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_table_description(&self) -> &::std::option::Option<dynamodb::types::ImportTableDescription> {
    &self.import_table_description
}
    /// Consumes the builder and constructs a [`ImportTableOutput`](crate::operation::operation::ImportTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::import_table::ImportTableOutput {
            import_table_description: self.import_table_description,
        })
    }
}
