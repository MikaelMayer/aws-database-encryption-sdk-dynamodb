// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_endpoints::DescribeEndpointsResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsResponse::DescribeEndpointsResponse {
        Endpoints: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.endpoints.clone().unwrap(),
    |e| dynamodb::conversions::endpoint::to_dafny(e.clone())
,
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeEndpointsResponse,
    >,
) -> crate::operation::describe_endpoints::DescribeEndpointsResponse {
    crate::operation::describe_endpoints::DescribeEndpointsResponse::builder()
        .set_endpoints(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.Endpoints(),
    |e| dynamodb::conversions::endpoint::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
