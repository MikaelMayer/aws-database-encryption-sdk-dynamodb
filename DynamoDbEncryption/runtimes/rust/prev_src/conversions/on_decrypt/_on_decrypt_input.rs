// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::on_decrypt::OnDecryptInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnDecryptInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnDecryptInput::OnDecryptInput {
        materials: material_providers::conversions::decryption_materials::to_dafny(&value.materials.clone().unwrap())
,
 encryptedDataKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_keys.clone().unwrap(),
    |e| material_providers::conversions::encrypted_data_key::to_dafny(e.clone())
,
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::OnDecryptInput,
    >,
) -> crate::operation::on_decrypt::OnDecryptInput {
    crate::operation::on_decrypt::OnDecryptInput::builder()
        .set_materials(Some( material_providers::conversions::decryption_materials::from_dafny(dafny_value.materials().clone())
 ))
 .set_encrypted_data_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedDataKeys(),
    |e| material_providers::conversions::encrypted_data_key::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
