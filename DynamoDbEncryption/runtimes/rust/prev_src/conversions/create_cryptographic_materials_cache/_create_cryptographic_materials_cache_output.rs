// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheOutput::CreateCryptographicMaterialsCacheOutput {
        materialsCache: material_providers::conversions::cryptographic_materials_cache::to_dafny(value.materials_cache.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheOutput,
    >,
) -> crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput {
    crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheOutput::builder()
        .set_materials_cache(Some( material_providers::conversions::cryptographic_materials_cache::from_dafny(dafny_value.materialsCache().clone())
 ))
        .build()
        .unwrap()
}
