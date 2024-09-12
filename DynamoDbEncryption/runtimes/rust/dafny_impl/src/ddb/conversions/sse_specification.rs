// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::SseSpecification,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification::SSESpecification {
        Enabled: crate::ddb::standard_library_conversions::obool_to_dafny(&value.enabled),
 SSEType: ::std::rc::Rc::new(match &value.sse_type {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::sse_type::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 KMSMasterKeyId: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.kms_master_key_id),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification,
    >,
) -> aws_sdk_dynamodb::types::SseSpecification {
    aws_sdk_dynamodb::types::SseSpecification::builder()
          .set_enabled(crate::ddb::standard_library_conversions::obool_from_dafny(dafny_value.Enabled().clone()))
 .set_sse_type(match &**dafny_value.SSEType() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::sse_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_kms_master_key_id(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.KMSMasterKeyId().clone()))
          .build()

}
