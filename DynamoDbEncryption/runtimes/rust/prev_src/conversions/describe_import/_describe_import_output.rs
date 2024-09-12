// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_import::DescribeImportOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeImportOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeImportOutput::DescribeImportOutput {
        ImportTableDescription: dynamodb::conversions::import_table_description::to_dafny(&value.import_table_description.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeImportOutput,
    >,
) -> crate::operation::describe_import::DescribeImportOutput {
    crate::operation::describe_import::DescribeImportOutput::builder()
        .set_import_table_description(Some( dynamodb::conversions::import_table_description::from_dafny(dafny_value.ImportTableDescription().clone())
 ))
        .build()
        .unwrap()
}
