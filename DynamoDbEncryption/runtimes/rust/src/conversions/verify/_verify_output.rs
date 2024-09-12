// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::verify::VerifyResponse,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::VerifyResponse,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::VerifyResponse::VerifyResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SignatureValid: crate::standard_library_conversions::obool_to_dafny(&value.signature_valid),
 SigningAlgorithm: ::std::rc::Rc::new(match &value.signing_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: kms::conversions::signing_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::VerifyResponse,
    >,
) -> crate::operation::verify::VerifyResponse {
    crate::operation::verify::VerifyResponse::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_signature_valid(crate::standard_library_conversions::obool_from_dafny(dafny_value.SignatureValid().clone()))
 .set_signing_algorithm(match &**dafny_value.SigningAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        kms::conversions::signing_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
