// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Unit {

}
impl Unit {

}
impl Unit {
    /// Creates a new builder-style object to manufacture [`Unit`](crate::operation::tag_resource::builders::Unit).
    pub fn builder() -> crate::operation::tag_resource::builders::UnitBuilder {
        crate::operation::tag_resource::builders::UnitBuilder::default()
    }
}

/// A builder for [`Unit`](crate::operation::operation::Unit).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UnitBuilder {

}
impl UnitBuilder {

    /// Consumes the builder and constructs a [`Unit`](crate::operation::operation::Unit).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::tag_resource::Unit,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::tag_resource::Unit {

        })
    }
}
