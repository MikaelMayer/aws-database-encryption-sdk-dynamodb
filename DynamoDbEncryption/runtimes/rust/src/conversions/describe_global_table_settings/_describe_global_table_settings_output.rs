// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsOutput::DescribeGlobalTableSettingsOutput {
        GlobalTableName: crate::standard_library_conversions::ostring_to_dafny(&value.global_table_name),
 ReplicaSettings: ::std::rc::Rc::new(match &value.replica_settings {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dynamodb::conversions::replica_settings_description::to_dafny(e.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DescribeGlobalTableSettingsOutput,
    >,
) -> crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput {
    crate::operation::describe_global_table_settings::DescribeGlobalTableSettingsOutput::builder()
        .set_global_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableName().clone()))
 .set_replica_settings(match (*dafny_value.ReplicaSettings()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dynamodb::conversions::replica_settings_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
