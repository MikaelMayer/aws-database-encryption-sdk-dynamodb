// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl CreateGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_description(&self) -> &::std::option::Option<dynamodb::types::GlobalTableDescription> {
    &self.global_table_description
}
}
impl CreateGlobalTableOutput {
    /// Creates a new builder-style object to manufacture [`CreateGlobalTableOutput`](crate::operation::create_global_table::builders::CreateGlobalTableOutput).
    pub fn builder() -> crate::operation::create_global_table::builders::CreateGlobalTableOutputBuilder {
        crate::operation::create_global_table::builders::CreateGlobalTableOutputBuilder::default()
    }
}

/// A builder for [`CreateGlobalTableOutput`](crate::operation::operation::CreateGlobalTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateGlobalTableOutputBuilder {
    pub(crate) global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl CreateGlobalTableOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_description(mut self, input: impl ::std::convert::Into<dynamodb::types::GlobalTableDescription>) -> Self {
    self.global_table_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_global_table_description(mut self, input: ::std::option::Option<dynamodb::types::GlobalTableDescription>) -> Self {
    self.global_table_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_global_table_description(&self) -> &::std::option::Option<dynamodb::types::GlobalTableDescription> {
    &self.global_table_description
}
    /// Consumes the builder and constructs a [`CreateGlobalTableOutput`](crate::operation::operation::CreateGlobalTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_global_table::CreateGlobalTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_global_table::CreateGlobalTableOutput {
            global_table_description: self.global_table_description,
        })
    }
}
