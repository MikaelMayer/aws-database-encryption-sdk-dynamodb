// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::decrypt_materials::DecryptMaterialsOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsOutput::DecryptMaterialsOutput {
        decryptionMaterials: material_providers::conversions::decryption_materials::to_dafny(&value.decryption_materials.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptMaterialsOutput,
    >,
) -> crate::operation::decrypt_materials::DecryptMaterialsOutput {
    crate::operation::decrypt_materials::DecryptMaterialsOutput::builder()
        .set_decryption_materials(Some( material_providers::conversions::decryption_materials::from_dafny(dafny_value.decryptionMaterials().clone())
 ))
        .build()
        .unwrap()
}
