// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeEndpointsRequest {

}
impl DescribeEndpointsRequest {

}
impl DescribeEndpointsRequest {
    /// Creates a new builder-style object to manufacture [`DescribeEndpointsRequest`](crate::operation::describe_endpoints::builders::DescribeEndpointsRequest).
    pub fn builder() -> crate::operation::describe_endpoints::builders::DescribeEndpointsRequestBuilder {
        crate::operation::describe_endpoints::builders::DescribeEndpointsRequestBuilder::default()
    }
}

/// A builder for [`DescribeEndpointsRequest`](crate::operation::operation::DescribeEndpointsRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeEndpointsRequestBuilder {

}
impl DescribeEndpointsRequestBuilder {

    /// Consumes the builder and constructs a [`DescribeEndpointsRequest`](crate::operation::operation::DescribeEndpointsRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_endpoints::DescribeEndpointsRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_endpoints::DescribeEndpointsRequest {

        })
    }
}
