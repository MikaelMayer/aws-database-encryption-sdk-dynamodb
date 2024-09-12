// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExportTableToPointInTimeOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExportTableToPointInTimeOutput::ExportTableToPointInTimeOutput {
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExportTableToPointInTimeOutput,
    >,
) -> crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput {
    crate::operation::export_table_to_point_in_time::ExportTableToPointInTimeOutput::builder()
        .set_export_description(match (*dafny_value.ExportDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::export_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
