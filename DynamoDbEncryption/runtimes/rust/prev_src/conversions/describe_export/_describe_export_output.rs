// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_export::DescribeExportOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeExportOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeExportOutput::DescribeExportOutput {
        ExportDescription: ::std::rc::Rc::new(match &value.export_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::export_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeExportOutput,
    >,
) -> crate::operation::describe_export::DescribeExportOutput {
    crate::operation::describe_export::DescribeExportOutput::builder()
        .set_export_description(match (*dafny_value.ExportDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::export_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
