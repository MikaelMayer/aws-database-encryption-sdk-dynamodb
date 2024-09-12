// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EnableKinesisStreamingDestinationOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EnableKinesisStreamingDestinationOutput::EnableKinesisStreamingDestinationOutput {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 StreamArn: crate::standard_library_conversions::ostring_to_dafny(&value.stream_arn),
 DestinationStatus: ::std::rc::Rc::new(match &value.destination_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::destination_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::EnableKinesisStreamingDestinationOutput,
    >,
) -> crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput {
    crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::builder()
        .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_stream_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.StreamArn().clone()))
 .set_destination_status(match &**dafny_value.DestinationStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        dynamodb::conversions::destination_status::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
