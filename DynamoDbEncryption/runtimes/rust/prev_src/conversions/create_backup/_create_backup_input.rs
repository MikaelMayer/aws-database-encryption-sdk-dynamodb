// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_backup::CreateBackupInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupInput::CreateBackupInput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 BackupName: crate::standard_library_conversions::ostring_to_dafny(&value.backup_name) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateBackupInput,
    >,
) -> crate::operation::create_backup::CreateBackupInput {
    crate::operation::create_backup::CreateBackupInput::builder()
        .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_backup_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.BackupName()) ))
        .build()
        .unwrap()
}
