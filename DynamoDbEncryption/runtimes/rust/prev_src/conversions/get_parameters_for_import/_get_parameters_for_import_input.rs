// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_parameters_for_import::GetParametersForImportRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetParametersForImportRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetParametersForImportRequest::GetParametersForImportRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 WrappingAlgorithm: kms::conversions::algorithm_spec::to_dafny(value.wrapping_algorithm.clone().unwrap()),
 WrappingKeySpec: kms::conversions::wrapping_key_spec::to_dafny(value.wrapping_key_spec.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::GetParametersForImportRequest,
    >,
) -> crate::operation::get_parameters_for_import::GetParametersForImportRequest {
    crate::operation::get_parameters_for_import::GetParametersForImportRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_wrapping_algorithm(Some( kms::conversions::algorithm_spec::from_dafny(dafny_value.WrappingAlgorithm()) ))
 .set_wrapping_key_spec(Some( kms::conversions::wrapping_key_spec::from_dafny(dafny_value.WrappingKeySpec()) ))
        .build()
        .unwrap()
}
