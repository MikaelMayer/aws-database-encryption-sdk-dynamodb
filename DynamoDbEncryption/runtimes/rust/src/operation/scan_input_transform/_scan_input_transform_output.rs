// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScanInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub transformed_input: ::std::option::Option<dynamodb::types::ScanInput>,
}
impl ScanInputTransformOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(&self) -> &::std::option::Option<dynamodb::types::ScanInput> {
    &self.transformed_input
}
}
impl ScanInputTransformOutput {
    /// Creates a new builder-style object to manufacture [`ScanInputTransformOutput`](crate::operation::scan_input_transform::builders::ScanInputTransformOutput).
    pub fn builder() -> crate::operation::scan_input_transform::builders::ScanInputTransformOutputBuilder {
        crate::operation::scan_input_transform::builders::ScanInputTransformOutputBuilder::default()
    }
}

/// A builder for [`ScanInputTransformOutput`](crate::operation::operation::ScanInputTransformOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScanInputTransformOutputBuilder {
    pub(crate) transformed_input: ::std::option::Option<dynamodb::types::ScanInput>,
}
impl ScanInputTransformOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn transformed_input(mut self, input: impl ::std::convert::Into<dynamodb::types::ScanInput>) -> Self {
    self.transformed_input = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transformed_input(mut self, input: ::std::option::Option<dynamodb::types::ScanInput>) -> Self {
    self.transformed_input = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transformed_input(&self) -> &::std::option::Option<dynamodb::types::ScanInput> {
    &self.transformed_input
}
    /// Consumes the builder and constructs a [`ScanInputTransformOutput`](crate::operation::operation::ScanInputTransformOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::scan_input_transform::ScanInputTransformOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::scan_input_transform::ScanInputTransformOutput {
            transformed_input: self.transformed_input,
        })
    }
}