// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreRequest::DisconnectCustomKeyStoreRequest {
        CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id) .Extract(),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreRequest,
    >,
) -> crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreRequest {
    crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreRequest::builder()
        .set_custom_key_store_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.CustomKeyStoreId()) ))
        .build()
        .unwrap()
}
