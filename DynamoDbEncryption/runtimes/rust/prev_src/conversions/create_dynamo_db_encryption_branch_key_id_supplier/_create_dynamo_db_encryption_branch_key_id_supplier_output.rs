// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput {
        branchKeyIdSupplier: material_providers::conversions::branch_key_id_supplier::to_dafny(value.branch_key_id_supplier.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput,
    >,
) -> crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput {
    crate::operation::create_dynamo_db_encryption_branch_key_id_supplier::CreateDynamoDbEncryptionBranchKeyIdSupplierOutput::builder()
        .set_branch_key_id_supplier(Some( material_providers::conversions::branch_key_id_supplier::from_dafny(dafny_value.branchKeyIdSupplier().clone())
 ))
        .build()
        .unwrap()
}
