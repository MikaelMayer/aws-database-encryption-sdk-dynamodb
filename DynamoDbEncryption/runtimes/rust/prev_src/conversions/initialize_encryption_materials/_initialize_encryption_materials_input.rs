// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::initialize_encryption_materials::InitializeEncryptionMaterialsInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::InitializeEncryptionMaterialsInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::InitializeEncryptionMaterialsInput::InitializeEncryptionMaterialsInput {
        algorithmSuiteId: material_providers::conversions::algorithm_suite_id::to_dafny(&value.algorithm_suite_id.clone().unwrap())
,
 encryptionContext: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.encryption_context.clone().unwrap(),
    |k| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&k.as_bytes().to_vec(), |b| *b)),
    |v| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v.as_bytes().to_vec(), |b| *b)),
)
,
 requiredEncryptionContextKeys: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.required_encryption_context_keys.clone().unwrap(),
    |e| ::std::rc::Rc::new(dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&e.as_bytes().to_vec(), |b| *b)),
)
,
 signingKey: crate::standard_library_conversions::oblob_to_dafny(&value.signing_key),
 verificationKey: crate::standard_library_conversions::oblob_to_dafny(&value.verification_key),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::InitializeEncryptionMaterialsInput,
    >,
) -> crate::operation::initialize_encryption_materials::InitializeEncryptionMaterialsInput {
    crate::operation::initialize_encryption_materials::InitializeEncryptionMaterialsInput::builder()
        .set_algorithm_suite_id(Some( material_providers::conversions::algorithm_suite_id::from_dafny(dafny_value.algorithmSuiteId().clone())
 ))
 .set_encryption_context(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.encryptionContext(),
    |k| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&k.as_ref(), |b| *b)).unwrap(),
    |v| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&v.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_required_encryption_context_keys(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.requiredEncryptionContextKeys(),
    |e| ::std::string::String::from_utf8(dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&e.as_ref(), |b| *b)).unwrap(),
)
 ))
 .set_signing_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.signingKey().clone()))
 .set_verification_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.verificationKey().clone()))
        .build()
        .unwrap()
}
