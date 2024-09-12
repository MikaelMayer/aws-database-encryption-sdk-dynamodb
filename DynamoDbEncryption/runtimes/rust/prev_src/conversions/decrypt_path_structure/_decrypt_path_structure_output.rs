// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::decrypt_path_structure::DecryptPathStructureOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptPathStructureOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptPathStructureOutput::DecryptPathStructureOutput {
        plaintextStructure: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.plaintext_structure.clone().unwrap(),
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
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DecryptPathStructureOutput,
    >,
) -> crate::operation::decrypt_path_structure::DecryptPathStructureOutput {
    crate::operation::decrypt_path_structure::DecryptPathStructureOutput::builder()
        .set_plaintext_structure(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.plaintextStructure(),
    |e| structured_encryption::conversions::crypto_item::from_dafny(e.clone())
,
)
 ))
 .set_parsed_header(Some( structured_encryption::conversions::parsed_header::from_dafny(dafny_value.parsedHeader().clone())
 ))
        .build()
        .unwrap()
}
