// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeEndpointsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub endpoints: ::std::option::Option<::std::vec::Vec<dynamodb::types::Endpoint>>,
}
impl DescribeEndpointsResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn endpoints(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Endpoint>> {
    &self.endpoints
}
}
impl DescribeEndpointsResponse {
    /// Creates a new builder-style object to manufacture [`DescribeEndpointsResponse`](crate::operation::describe_endpoints::builders::DescribeEndpointsResponse).
    pub fn builder() -> crate::operation::describe_endpoints::builders::DescribeEndpointsResponseBuilder {
        crate::operation::describe_endpoints::builders::DescribeEndpointsResponseBuilder::default()
    }
}

/// A builder for [`DescribeEndpointsResponse`](crate::operation::operation::DescribeEndpointsResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeEndpointsResponseBuilder {
    pub(crate) endpoints: ::std::option::Option<::std::vec::Vec<dynamodb::types::Endpoint>>,
}
impl DescribeEndpointsResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn endpoints(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::Endpoint>>) -> Self {
    self.endpoints = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_endpoints(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::Endpoint>>) -> Self {
    self.endpoints = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_endpoints(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::Endpoint>> {
    &self.endpoints
}
    /// Consumes the builder and constructs a [`DescribeEndpointsResponse`](crate::operation::operation::DescribeEndpointsResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_endpoints::DescribeEndpointsResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_endpoints::DescribeEndpointsResponse {
            endpoints: self.endpoints,
        })
    }
}
