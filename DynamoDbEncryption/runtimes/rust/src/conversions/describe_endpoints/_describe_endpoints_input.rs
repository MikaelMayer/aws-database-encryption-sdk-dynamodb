// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_endpoints::DescribeEndpointsRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsRequest::DescribeEndpointsRequest {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsRequest,
    >,
) -> crate::operation::describe_endpoints::DescribeEndpointsRequest {
    crate::operation::describe_endpoints::DescribeEndpointsRequest::builder()

        .build()
        .unwrap()
}
