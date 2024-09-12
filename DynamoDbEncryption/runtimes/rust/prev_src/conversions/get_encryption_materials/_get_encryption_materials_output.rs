// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsOutput::GetEncryptionMaterialsOutput {
        encryptionMaterials: material_providers::conversions::encryption_materials::to_dafny(&value.encryption_materials.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptionMaterialsOutput,
    >,
) -> crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput {
    crate::operation::get_encryption_materials::GetEncryptionMaterialsOutput::builder()
        .set_encryption_materials(Some( material_providers::conversions::encryption_materials::from_dafny(dafny_value.encryptionMaterials().clone())
 ))
        .build()
        .unwrap()
}
