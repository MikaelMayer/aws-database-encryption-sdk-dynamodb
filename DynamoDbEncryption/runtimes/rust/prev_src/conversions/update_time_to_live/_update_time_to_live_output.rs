// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::update_time_to_live::UpdateTimeToLiveOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateTimeToLiveOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateTimeToLiveOutput::UpdateTimeToLiveOutput {
        TimeToLiveSpecification: ::std::rc::Rc::new(match &value.time_to_live_specification {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::time_to_live_specification::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::UpdateTimeToLiveOutput,
    >,
) -> crate::operation::update_time_to_live::UpdateTimeToLiveOutput {
    crate::operation::update_time_to_live::UpdateTimeToLiveOutput::builder()
        .set_time_to_live_specification(match (*dafny_value.TimeToLiveSpecification()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::time_to_live_specification::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
