// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_cache_entry::GetCacheEntryInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryInput::GetCacheEntryInput {
        identifier: crate::standard_library_conversions::oblob_to_dafny(&value.identifier).Extract(),
 bytesUsed: crate::standard_library_conversions::olong_to_dafny(&value.bytes_used),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetCacheEntryInput,
    >,
) -> crate::operation::get_cache_entry::GetCacheEntryInput {
    crate::operation::get_cache_entry::GetCacheEntryInput::builder()
        .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
 .set_bytes_used(crate::standard_library_conversions::olong_from_dafny(dafny_value.bytesUsed().clone()))
        .build()
        .unwrap()
}
