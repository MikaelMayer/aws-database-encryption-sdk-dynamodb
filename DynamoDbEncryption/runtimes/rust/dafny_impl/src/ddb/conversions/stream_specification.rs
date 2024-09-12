// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::StreamSpecification,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification::StreamSpecification {
        StreamEnabled: value.stream_enabled,
 StreamViewType: ::std::rc::Rc::new(match &value.stream_view_type {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::stream_view_type::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification,
    >,
) -> aws_sdk_dynamodb::types::StreamSpecification {
    aws_sdk_dynamodb::types::StreamSpecification::builder()
          .set_stream_enabled(Some( dafny_value.StreamEnabled() .clone() ))
 .set_stream_view_type(match &**dafny_value.StreamViewType() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::stream_view_type::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
          .unwrap()
}
