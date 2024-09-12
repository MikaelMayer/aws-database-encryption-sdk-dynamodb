// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreResponse::DisconnectCustomKeyStoreResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DisconnectCustomKeyStoreResponse,
    >,
) -> crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse {
    crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse::builder()

        .build()
        .unwrap()
}
