// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl UpdateGlobalTableOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn global_table_description(&self) -> &::std::option::Option<dynamodb::types::GlobalTableDescription> {
    &self.global_table_description
}
}
impl UpdateGlobalTableOutput {
    /// Creates a new builder-style object to manufacture [`UpdateGlobalTableOutput`](crate::operation::update_global_table::builders::UpdateGlobalTableOutput).
    pub fn builder() -> crate::operation::update_global_table::builders::UpdateGlobalTableOutputBuilder {
        crate::operation::update_global_table::builders::UpdateGlobalTableOutputBuilder::default()
    }
}

/// A builder for [`UpdateGlobalTableOutput`](crate::operation::operation::UpdateGlobalTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateGlobalTableOutputBuilder {
    pub(crate) global_table_description: ::std::option::Option<dynamodb::types::GlobalTableDescription>,
}
impl UpdateGlobalTableOutputBuilder {
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
    /// Consumes the builder and constructs a [`UpdateGlobalTableOutput`](crate::operation::operation::UpdateGlobalTableOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_global_table::UpdateGlobalTableOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_global_table::UpdateGlobalTableOutput {
            global_table_description: self.global_table_description,
        })
    }
}
