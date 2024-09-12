// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::delete_backup::DeleteBackupOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteBackupOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteBackupOutput::DeleteBackupOutput {
        BackupDescription: ::std::rc::Rc::new(match &value.backup_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::backup_description::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteBackupOutput,
    >,
) -> crate::operation::delete_backup::DeleteBackupOutput {
    crate::operation::delete_backup::DeleteBackupOutput::builder()
        .set_backup_description(match (*dafny_value.BackupDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::backup_description::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
