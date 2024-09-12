// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::delete_cache_entry::DeleteCacheEntryInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCacheEntryInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCacheEntryInput::DeleteCacheEntryInput {
        identifier: crate::standard_library_conversions::oblob_to_dafny(&value.identifier).Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCacheEntryInput,
    >,
) -> crate::operation::delete_cache_entry::DeleteCacheEntryInput {
    crate::operation::delete_cache_entry::DeleteCacheEntryInput::builder()
        .set_identifier(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.identifier().clone())))
        .build()
        .unwrap()
}
