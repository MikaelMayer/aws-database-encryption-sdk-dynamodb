// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCustomKeyStoreResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCustomKeyStoreResponse::DeleteCustomKeyStoreResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeleteCustomKeyStoreResponse,
    >,
) -> crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse {
    crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse::builder()

        .build()
        .unwrap()
}
