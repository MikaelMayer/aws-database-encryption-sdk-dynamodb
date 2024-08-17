// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::re_encrypt::ReEncryptOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse,
> {
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse::ReEncryptResponse {
        CiphertextBlob: crate::kms::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 SourceKeyId: crate::kms::standard_library_conversions::ostring_to_dafny(&value.source_key_id),
 KeyId: crate::kms::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SourceEncryptionAlgorithm: ::std::rc::Rc::new(match &value.source_encryption_algorithm {
    Some(x) => crate::Wrappers::Option::Some { value: crate::kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 DestinationEncryptionAlgorithm: ::std::rc::Rc::new(match &value.destination_encryption_algorithm {
    Some(x) => crate::Wrappers::Option::Some { value: crate::kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
    })
}
