// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheInput::CreateCryptographicMaterialsCacheInput {
        cache: material_providers::conversions::cache_type::to_dafny(&value.cache.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCryptographicMaterialsCacheInput,
    >,
) -> crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput {
    crate::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput::builder()
        .set_cache(Some( material_providers::conversions::cache_type::from_dafny(dafny_value.cache().clone())
 ))
        .build()
        .unwrap()
}
