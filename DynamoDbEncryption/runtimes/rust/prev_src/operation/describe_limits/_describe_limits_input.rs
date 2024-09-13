// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLimitsInput {

}
impl DescribeLimitsInput {

}
impl DescribeLimitsInput {
    /// Creates a new builder-style object to manufacture [`DescribeLimitsInput`](crate::operation::describe_limits::builders::DescribeLimitsInput).
    pub fn builder() -> crate::operation::describe_limits::builders::DescribeLimitsInputBuilder {
        crate::operation::describe_limits::builders::DescribeLimitsInputBuilder::default()
    }
}

/// A builder for [`DescribeLimitsInput`](crate::operation::operation::DescribeLimitsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeLimitsInputBuilder {

}
impl DescribeLimitsInputBuilder {

    /// Consumes the builder and constructs a [`DescribeLimitsInput`](crate::operation::operation::DescribeLimitsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_limits::DescribeLimitsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_limits::DescribeLimitsInput {

        })
    }
}