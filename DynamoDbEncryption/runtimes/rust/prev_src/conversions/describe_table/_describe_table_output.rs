// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_table::DescribeTableOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTableOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTableOutput::DescribeTableOutput {
        Table: ::std::rc::Rc::new(match &value.table {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::table_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeTableOutput,
    >,
) -> crate::operation::describe_table::DescribeTableOutput {
    crate::operation::describe_table::DescribeTableOutput::builder()
        .set_table(match (*dafny_value.Table()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::table_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
