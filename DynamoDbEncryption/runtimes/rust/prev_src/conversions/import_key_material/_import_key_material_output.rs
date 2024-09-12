// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::import_key_material::ImportKeyMaterialResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ImportKeyMaterialResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ImportKeyMaterialResponse::ImportKeyMaterialResponse {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ImportKeyMaterialResponse,
    >,
) -> crate::operation::import_key_material::ImportKeyMaterialResponse {
    crate::operation::import_key_material::ImportKeyMaterialResponse::builder()

        .build()
        .unwrap()
}
