// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::re_encrypt::ReEncryptResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReEncryptResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReEncryptResponse::ReEncryptResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 SourceKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.source_key_id),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SourceEncryptionAlgorithm: ::std::rc::Rc::new(match &value.source_encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DestinationEncryptionAlgorithm: ::std::rc::Rc::new(match &value.destination_encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ReEncryptResponse,
    >,
) -> crate::operation::re_encrypt::ReEncryptResponse {
    crate::operation::re_encrypt::ReEncryptResponse::builder()
        .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextBlob().clone()))
 .set_source_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.SourceKeyId().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_source_encryption_algorithm(match &**dafny_value.SourceEncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_destination_encryption_algorithm(match &**dafny_value.DestinationEncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
