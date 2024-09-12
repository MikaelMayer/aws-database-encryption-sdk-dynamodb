// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_cache_entry::GetCacheEntryOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryOutput::GetCacheEntryOutput {
        materials: material_providers::conversions::materials::to_dafny(&value.materials.clone().unwrap())
,
 creationTime: crate::standard_library_conversions::olong_to_dafny(&value.creation_time),
 expiryTime: crate::standard_library_conversions::olong_to_dafny(&value.expiry_time),
 messagesUsed: value.messages_used.clone(),
 bytesUsed: value.bytes_used.clone(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryOutput,
    >,
) -> crate::operation::get_cache_entry::GetCacheEntryOutput {
    crate::operation::get_cache_entry::GetCacheEntryOutput::builder()
        .set_materials(Some( material_providers::conversions::materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_creation_time(Some( dafny_value.creationTime() .clone() ))
 .set_expiry_time(Some( dafny_value.expiryTime() .clone() ))
 .set_messages_used(crate::standard_library_conversions::oint_from_dafny(dafny_value.messagesUsed().clone()))
 .set_bytes_used(crate::standard_library_conversions::oint_from_dafny(dafny_value.bytesUsed().clone()))
        .build()
        .unwrap()
}
