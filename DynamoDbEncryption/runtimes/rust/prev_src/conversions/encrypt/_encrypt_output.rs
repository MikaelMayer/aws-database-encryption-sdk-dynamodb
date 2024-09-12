// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::encrypt::EncryptResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptResponse::EncryptResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 EncryptionAlgorithm: ::std::rc::Rc::new(match &value.encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EncryptResponse,
    >,
) -> crate::operation::encrypt::EncryptResponse {
    crate::operation::encrypt::EncryptResponse::builder()
        .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextBlob().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_encryption_algorithm(match &**dafny_value.EncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
