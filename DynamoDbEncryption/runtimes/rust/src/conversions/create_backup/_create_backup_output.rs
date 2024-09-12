// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_backup::CreateBackupOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupOutput::CreateBackupOutput {
        BackupDetails: ::std::rc::Rc::new(match &value.backup_details {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::backup_details::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupOutput,
    >,
) -> crate::operation::create_backup::CreateBackupOutput {
    crate::operation::create_backup::CreateBackupOutput::builder()
        .set_backup_details(match (*dafny_value.BackupDetails()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::backup_details::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
