// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyWithoutPlaintextResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyWithoutPlaintextResponse::GenerateDataKeyWithoutPlaintextResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GenerateDataKeyWithoutPlaintextResponse,
    >,
) -> crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse {
    crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse::builder()
        .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextBlob().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
        .build()
        .unwrap()
}
