// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_data_key::GenerateDataKeyResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyResponse::GenerateDataKeyResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 Plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyResponse,
    >,
) -> crate::operation::generate_data_key::GenerateDataKeyResponse {
    crate::operation::generate_data_key::GenerateDataKeyResponse::builder()
        .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextBlob().clone()))
 .set_plaintext(crate::standard_library_conversions::oblob_from_dafny(dafny_value.Plaintext().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextForRecipient().clone()))
        .build()
        .unwrap()
}
