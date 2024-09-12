// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AlgorithmSuiteInfo,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AlgorithmSuiteInfo::AlgorithmSuiteInfo {
        id: material_providers::conversions::algorithm_suite_id::to_dafny(&value.id.clone().unwrap())
,
 binaryId: crate::standard_library_conversions::oblob_to_dafny(&value.binary_id).Extract(),
 messageVersion: value.message_version.clone(),
 encrypt: material_providers::conversions::encrypt::to_dafny(&value.encrypt.clone().unwrap())
,
 kdf: material_providers::conversions::derivation_algorithm::to_dafny(&value.kdf.clone().unwrap())
,
 commitment: material_providers::conversions::derivation_algorithm::to_dafny(&value.commitment.clone().unwrap())
,
 signature: material_providers::conversions::signature_algorithm::to_dafny(&value.signature.clone().unwrap())
,
 symmetricSignature: material_providers::conversions::symmetric_signature_algorithm::to_dafny(&value.symmetric_signature.clone().unwrap())
,
 edkWrapping: material_providers::conversions::edk_wrapping_algorithm::to_dafny(&value.edk_wrapping.clone().unwrap())
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::AlgorithmSuiteInfo,
    >,
) -> crate::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo {
    crate::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo::builder()
        .set_id(Some( material_providers::conversions::algorithm_suite_id::from_dafny(dafny_value.id().clone())
 ))
 .set_binary_id(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.binaryId().clone())))
 .set_message_version(crate::standard_library_conversions::oint_from_dafny(dafny_value.messageVersion().clone()))
 .set_encrypt(Some( material_providers::conversions::encrypt::from_dafny(dafny_value.encrypt().clone())
 ))
 .set_kdf(Some( material_providers::conversions::derivation_algorithm::from_dafny(dafny_value.kdf().clone())
 ))
 .set_commitment(Some( material_providers::conversions::derivation_algorithm::from_dafny(dafny_value.commitment().clone())
 ))
 .set_signature(Some( material_providers::conversions::signature_algorithm::from_dafny(dafny_value.signature().clone())
 ))
 .set_symmetric_signature(Some( material_providers::conversions::symmetric_signature_algorithm::from_dafny(dafny_value.symmetricSignature().clone())
 ))
 .set_edk_wrapping(Some( material_providers::conversions::edk_wrapping_algorithm::from_dafny(dafny_value.edkWrapping().clone())
 ))
        .build()
        .unwrap()
}
