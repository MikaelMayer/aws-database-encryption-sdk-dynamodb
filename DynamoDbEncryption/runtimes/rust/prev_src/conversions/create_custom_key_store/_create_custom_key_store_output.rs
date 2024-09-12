// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCustomKeyStoreResponse::CreateCustomKeyStoreResponse {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::CreateCustomKeyStoreResponse,
    >,
) -> crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse {
    crate::operation::create_custom_key_store::CreateCustomKeyStoreResponse::builder()
        .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CustomKeyStoreId().clone()))
        .build()
        .unwrap()
}
