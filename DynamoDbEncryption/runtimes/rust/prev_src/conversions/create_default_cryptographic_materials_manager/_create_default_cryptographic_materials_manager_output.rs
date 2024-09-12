// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsManagerOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsManagerOutput::CreateCryptographicMaterialsManagerOutput {
        materialsManager: material_providers::conversions::cryptographic_materials_manager::to_dafny(value.materials_manager.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsManagerOutput,
    >,
) -> crate::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput {
    crate::operation::create_default_cryptographic_materials_manager::CreateCryptographicMaterialsManagerOutput::builder()
        .set_materials_manager(Some( material_providers::conversions::cryptographic_materials_manager::from_dafny(dafny_value.materialsManager().clone())
 ))
        .build()
        .unwrap()
}
