// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierInput::CreateDynamoDbEncryptionBranchKeyIdSupplierInput {
        ddbKeyBranchKeyIdSupplier: dynamo_db::conversions::dynamo_db_key_branch_key_id_supplier::to_dafny(value.ddb_key_branch_key_id_supplier.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierInput,
    >,
) -> crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierInput {
    crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierInput::builder()
        .set_ddb_key_branch_key_id_supplier(Some( dynamo_db::conversions::dynamo_db_key_branch_key_id_supplier::from_dafny(dafny_value.ddbKeyBranchKeyIdSupplier().clone())
 ))
        .build()
        .unwrap()
}