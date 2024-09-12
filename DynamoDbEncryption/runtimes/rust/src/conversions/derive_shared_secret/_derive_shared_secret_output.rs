// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::derive_shared_secret::DeriveSharedSecretResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretResponse::DeriveSharedSecretResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SharedSecret: crate::standard_library_conversions::oblob_to_dafny(&value.shared_secret),
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
 KeyAgreementAlgorithm: ::std::rc::Rc::new(match &value.key_agreement_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::key_agreement_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyOrigin: ::std::rc::Rc::new(match &value.key_origin {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::origin_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::DeriveSharedSecretResponse,
    >,
) -> crate::operation::derive_shared_secret::DeriveSharedSecretResponse {
    crate::operation::derive_shared_secret::DeriveSharedSecretResponse::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_shared_secret(crate::standard_library_conversions::oblob_from_dafny(dafny_value.SharedSecret().clone()))
 .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextForRecipient().clone()))
 .set_key_agreement_algorithm(match &**dafny_value.KeyAgreementAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::key_agreement_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_origin(match &**dafny_value.KeyOrigin() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::origin_type::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
