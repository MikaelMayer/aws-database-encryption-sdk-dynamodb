// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescriptionOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptedDataKeyDescriptionOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptedDataKeyDescriptionOutput::GetEncryptedDataKeyDescriptionOutput {
        EncryptedDataKeyDescriptionOutput: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_data_key_description_output.clone().unwrap(),
    |e| dynamo_db::conversions::encrypted_data_key_description::to_dafny(e.clone())
,
)
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetEncryptedDataKeyDescriptionOutput,
    >,
) -> crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescriptionOutput {
    crate::operation::get_encrypted_data_key_description::GetEncryptedDataKeyDescriptionOutput::builder()
        .set_encrypted_data_key_description_output(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.EncryptedDataKeyDescriptionOutput(),
    |e| dynamo_db::conversions::encrypted_data_key_description::from_dafny(e.clone())
,
)
 ))
        .build()
        .unwrap()
}
