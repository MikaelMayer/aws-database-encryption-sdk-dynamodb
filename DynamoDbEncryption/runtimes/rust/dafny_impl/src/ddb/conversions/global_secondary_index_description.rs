// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::GlobalSecondaryIndexDescription,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription {
        IndexName: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.index_name),
 KeySchema: ::std::rc::Rc::new(match &value.key_schema {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::key_schema_element::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 Projection: ::std::rc::Rc::new(match &value.projection {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::projection::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 IndexStatus: ::std::rc::Rc::new(match &value.index_status {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::index_status::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 Backfilling: crate::ddb::standard_library_conversions::obool_to_dafny(&value.backfilling),
 ProvisionedThroughput: ::std::rc::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::provisioned_throughput_description::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 IndexSizeBytes: crate::ddb::standard_library_conversions::olong_to_dafny(&value.index_size_bytes),
 ItemCount: crate::ddb::standard_library_conversions::olong_to_dafny(&value.item_count),
 IndexArn: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.index_arn),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription,
    >,
) -> aws_sdk_dynamodb::types::GlobalSecondaryIndexDescription {
    aws_sdk_dynamodb::types::GlobalSecondaryIndexDescription::builder()
          .set_index_name(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_key_schema(match (*dafny_value.KeySchema()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::key_schema_element::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_projection(match (*dafny_value.Projection()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::projection::from_dafny(value.clone())),
    _ => None,
}
)
 .set_index_status(match &**dafny_value.IndexStatus() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::index_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_backfilling(crate::ddb::standard_library_conversions::obool_from_dafny(dafny_value.Backfilling().clone()))
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::provisioned_throughput_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_index_size_bytes(crate::ddb::standard_library_conversions::olong_from_dafny(dafny_value.IndexSizeBytes().clone()))
 .set_item_count(crate::ddb::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
 .set_index_arn(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.IndexArn().clone()))
          .build()

}
