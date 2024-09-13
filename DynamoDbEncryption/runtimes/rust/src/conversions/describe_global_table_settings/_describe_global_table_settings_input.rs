// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsInput::DescribeGlobalTableSettingsInput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsInput,
    >,
) -> crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput {
    crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsInput::builder()
        .set_global_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.GlobalTableName()) ))
        .build()
        .unwrap()
}