// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::encrypt_structure::EncryptStructureOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptStructureOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptStructureOutput::EncryptStructureOutput {
        encryptedStructure: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encrypted_structure.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| structured_encryption::conversions::structured_data_terminal::to_dafny(v.clone())
,
)
,
 cryptoSchema: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.crypto_schema.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| structured_encryption::conversions::crypto_action::to_dafny(v.clone()),
)
,
 parsedHeader: structured_encryption::conversions::parsed_header::to_dafny(&value.parsed_header.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptStructureOutput,
    >,
) -> crate::operation::encrypt_structure::EncryptStructureOutput {
    crate::operation::encrypt_structure::EncryptStructureOutput::builder()
        .set_encrypted_structure(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptedStructure(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| structured_encryption::conversions::structured_data_terminal::from_dafny(v.clone())
,
)
 ))
 .set_crypto_schema(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.cryptoSchema(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| structured_encryption::conversions::crypto_action::from_dafny(v),
)
 ))
 .set_parsed_header(Some( structured_encryption::conversions::parsed_header::from_dafny(dafny_value.parsedHeader().clone())
 ))
        .build()
        .unwrap()
}
