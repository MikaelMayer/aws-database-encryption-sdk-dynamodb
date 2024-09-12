// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportTableToPointInTimeOutput {
    #[allow(missing_docs)] // documentation missing in model
pub export_description: ::std::option::Option<dynamodb::types::ExportDescription>,
}
impl ExportTableToPointInTimeOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_description(&self) -> &::std::option::Option<dynamodb::types::ExportDescription> {
    &self.export_description
}
}
impl ExportTableToPointInTimeOutput {
    /// Creates a new builder-style object to manufacture [`ExportTableToPointInTimeOutput`](crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutput).
    pub fn builder() -> crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutputBuilder {
        crate::operation::export_table_to_point_in_time::builders::ExportTableToPointInTimeOutputBuilder::default()
    }
}

/// A builder for [`ExportTableToPointInTimeOutput`](crate::operation::operation::ExportTableToPointInTimeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExportTableToPointInTimeOutputBuilder {
    pub(crate) export_description: ::std::option::Option<dynamodb::types::ExportDescription>,
}
impl ExportTableToPointInTimeOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn export_description(mut self, input: impl ::std::convert::Into<dynamodb::types::ExportDescription>) -> Self {
    self.export_description = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_export_description(mut self, input: ::std::option::Option<dynamodb::types::ExportDescription>) -> Self {
    self.export_description = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_export_description(&self) -> &::std::option::Option<dynamodb::types::ExportDescription> {
    &self.export_description
}
    /// Consumes the builder and constructs a [`ExportTableToPointInTimeOutput`](crate::operation::operation::ExportTableToPointInTimeOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput {
            export_description: self.export_description,
        })
    }
}
