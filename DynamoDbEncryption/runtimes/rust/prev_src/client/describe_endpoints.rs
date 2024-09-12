// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeEndpoints`](crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`DescribeEndpointsResponse`](crate::operation::describe_endpoints::DescribeEndpointsResponse) with field(s):
    ///   - [`endpoints(Option<::std::vec::Vec<dynamodb::types::Endpoint>>)`](crate::operation::describe_endpoints::DescribeEndpointsResponse::endpoints): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeEndpointsError>`](crate::operation::describe_endpoints::DescribeEndpointsError)
    pub fn describe_endpoints(&self) -> crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder {
        crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder::new(self.clone())
    }
}
