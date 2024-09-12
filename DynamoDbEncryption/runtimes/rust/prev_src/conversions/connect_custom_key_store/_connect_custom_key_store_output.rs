// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ConnectCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ConnectCustomKeyStoreResponse::ConnectCustomKeyStoreResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ConnectCustomKeyStoreResponse,
    >,
) -> crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse {
    crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse::builder()

        .build()
        .unwrap()
}
