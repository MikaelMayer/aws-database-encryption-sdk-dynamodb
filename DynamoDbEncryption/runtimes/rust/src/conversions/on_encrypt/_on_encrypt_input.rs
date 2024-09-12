// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::on_encrypt::OnEncryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnEncryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnEncryptInput::OnEncryptInput {
        materials: material_providers::conversions::encryption_materials::to_dafny(&value.materials.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnEncryptInput,
    >,
) -> crate::operation::on_encrypt::OnEncryptInput {
    crate::operation::on_encrypt::OnEncryptInput::builder()
        .set_materials(Some( material_providers::conversions::encryption_materials::from_dafny(dafny_value.materials().clone())
 ))
        .build()
        .unwrap()
}
