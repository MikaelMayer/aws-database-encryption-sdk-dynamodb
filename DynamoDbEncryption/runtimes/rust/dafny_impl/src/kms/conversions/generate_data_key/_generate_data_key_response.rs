// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_data_key::GenerateDataKeyOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyResponse,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyResponse::GenerateDataKeyResponse {
        CiphertextBlob: crate::kms::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 Plaintext: crate::kms::standard_library_conversions::oblob_to_dafny(&value.plaintext),
 KeyId: crate::kms::standard_library_conversions::ostring_to_dafny(&value.key_id),
 CiphertextForRecipient: crate::kms::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
    })
}