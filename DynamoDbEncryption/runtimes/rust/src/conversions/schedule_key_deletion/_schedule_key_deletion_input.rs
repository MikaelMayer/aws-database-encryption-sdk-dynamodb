// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ScheduleKeyDeletionRequest,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ScheduleKeyDeletionRequest::ScheduleKeyDeletionRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 PendingWindowInDays: crate::standard_library_conversions::oint_to_dafny(value.pending_window_in_days),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ScheduleKeyDeletionRequest,
    >,
) -> crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest {
    crate::operation::schedule_key_deletion::ScheduleKeyDeletionRequest::builder()
        .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_pending_window_in_days(crate::standard_library_conversions::oint_from_dafny(dafny_value.PendingWindowInDays().clone()))
        .build()
        .unwrap()
}
