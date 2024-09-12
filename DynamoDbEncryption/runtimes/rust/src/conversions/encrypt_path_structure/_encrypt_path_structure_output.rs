// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::encrypt_path_structure::EncryptPathStructureOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptPathStructureOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptPathStructureOutput::EncryptPathStructureOutput {
        encryptedStructure: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.encrypted_structure.clone().unwrap(),
    |e| structured_encryption::conversions::crypto_item::to_dafny(e.clone())
,
)
,
 parsedHeader: structured_encryption::conversions::parsed_header::to_dafny(&value.parsed_header.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptPathStructureOutput,
    >,
) -> crate::operation::encrypt_path_structure::EncryptPathStructureOutput {
    crate::operation::encrypt_path_structure::EncryptPathStructureOutput::builder()
        .set_encrypted_structure(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.encryptedStructure(),
    |e| structured_encryption::conversions::crypto_item::from_dafny(e.clone())
,
)
 ))
 .set_parsed_header(Some( structured_encryption::conversions::parsed_header::from_dafny(dafny_value.parsedHeader().clone())
 ))
        .build()
        .unwrap()
}
