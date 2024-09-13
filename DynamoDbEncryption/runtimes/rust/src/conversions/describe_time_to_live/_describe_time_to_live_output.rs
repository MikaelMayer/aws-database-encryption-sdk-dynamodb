// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_time_to_live::DescribeTimeToLiveOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTimeToLiveOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTimeToLiveOutput::DescribeTimeToLiveOutput {
        TimeToLiveDescription: ::std::rc::Rc::new(match &value.time_to_live_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::time_to_live_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTimeToLiveOutput,
    >,
) -> crate::operation::describe_time_to_live::DescribeTimeToLiveOutput {
    crate::operation::describe_time_to_live::DescribeTimeToLiveOutput::builder()
        .set_time_to_live_description(match (*dafny_value.TimeToLiveDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::time_to_live_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}