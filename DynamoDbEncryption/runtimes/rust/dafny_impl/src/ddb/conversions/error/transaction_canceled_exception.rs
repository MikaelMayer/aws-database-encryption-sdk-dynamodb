// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[allow(dead_code)]
pub fn to_dafny(
    value: aws_sdk_dynamodb::types::error::TransactionCanceledException,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TransactionCanceledException {
      Message: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.message),
 CancellationReasons: ::std::rc::Rc::new(match &value.cancellation_reasons {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::cancellation_reason::to_dafny(&e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
    }
  )
}
