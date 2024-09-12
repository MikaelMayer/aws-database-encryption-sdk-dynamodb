// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeContinuousBackupsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeContinuousBackupsOutput::DescribeContinuousBackupsOutput {
        ContinuousBackupsDescription: ::std::rc::Rc::new(match &value.continuous_backups_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::continuous_backups_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeContinuousBackupsOutput,
    >,
) -> crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput {
    crate::operation::describe_continuous_backups::DescribeContinuousBackupsOutput::builder()
        .set_continuous_backups_description(match (*dafny_value.ContinuousBackupsDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::continuous_backups_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
