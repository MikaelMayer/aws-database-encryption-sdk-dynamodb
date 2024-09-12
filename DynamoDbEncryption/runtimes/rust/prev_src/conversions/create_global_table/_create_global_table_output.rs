// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_global_table::CreateGlobalTableOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGlobalTableOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGlobalTableOutput::CreateGlobalTableOutput {
        GlobalTableDescription: ::std::rc::Rc::new(match &value.global_table_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::global_table_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateGlobalTableOutput,
    >,
) -> crate::operation::create_global_table::CreateGlobalTableOutput {
    crate::operation::create_global_table::CreateGlobalTableOutput::builder()
        .set_global_table_description(match (*dafny_value.GlobalTableDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::global_table_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
