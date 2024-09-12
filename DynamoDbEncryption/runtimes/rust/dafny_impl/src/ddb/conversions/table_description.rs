// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::TableDescription,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription::TableDescription {
        AttributeDefinitions: ::std::rc::Rc::new(match &value.attribute_definitions {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::attribute_definition::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 TableName: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.table_name),
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
 TableStatus: ::std::rc::Rc::new(match &value.table_status {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::table_status::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 CreationDateTime: crate::ddb::standard_library_conversions::otimestamp_to_dafny(&value.creation_date_time),
 ProvisionedThroughput: ::std::rc::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::provisioned_throughput_description::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 TableSizeBytes: crate::ddb::standard_library_conversions::olong_to_dafny(&value.table_size_bytes),
 ItemCount: crate::ddb::standard_library_conversions::olong_to_dafny(&value.item_count),
 TableArn: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 TableId: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.table_id),
 BillingModeSummary: ::std::rc::Rc::new(match &value.billing_mode_summary {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::billing_mode_summary::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 LocalSecondaryIndexes: ::std::rc::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::local_secondary_index_description::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 GlobalSecondaryIndexes: ::std::rc::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::global_secondary_index_description::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 StreamSpecification: ::std::rc::Rc::new(match &value.stream_specification {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::stream_specification::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 LatestStreamLabel: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.latest_stream_label),
 LatestStreamArn: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.latest_stream_arn),
 GlobalTableVersion: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.global_table_version),
 Replicas: ::std::rc::Rc::new(match &value.replicas {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::replica_description::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 RestoreSummary: ::std::rc::Rc::new(match &value.restore_summary {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::restore_summary::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 SSEDescription: ::std::rc::Rc::new(match &value.sse_description {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::sse_description::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 ArchivalSummary: ::std::rc::Rc::new(match &value.archival_summary {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::archival_summary::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 TableClassSummary: ::std::rc::Rc::new(match &value.table_class_summary {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::table_class_summary::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription,
    >,
) -> aws_sdk_dynamodb::types::TableDescription {
    aws_sdk_dynamodb::types::TableDescription::builder()
          .set_attribute_definitions(match (*dafny_value.AttributeDefinitions()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::attribute_definition::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_name(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
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
 .set_table_status(match &**dafny_value.TableStatus() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::table_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_creation_date_time(crate::ddb::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDateTime().clone()))
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::provisioned_throughput_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_table_size_bytes(crate::ddb::standard_library_conversions::olong_from_dafny(dafny_value.TableSizeBytes().clone()))
 .set_item_count(crate::ddb::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
 .set_table_arn(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.TableArn().clone()))
 .set_table_id(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.TableId().clone()))
 .set_billing_mode_summary(match (*dafny_value.BillingModeSummary()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::billing_mode_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::local_secondary_index_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::global_secondary_index_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_stream_specification(match (*dafny_value.StreamSpecification()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::stream_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_latest_stream_label(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.LatestStreamLabel().clone()))
 .set_latest_stream_arn(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.LatestStreamArn().clone()))
 .set_global_table_version(crate::ddb::standard_library_conversions::ostring_from_dafny(dafny_value.GlobalTableVersion().clone()))
 .set_replicas(match (*dafny_value.Replicas()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::replica_description::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_restore_summary(match (*dafny_value.RestoreSummary()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::restore_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_sse_description(match (*dafny_value.SSEDescription()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::sse_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_archival_summary(match (*dafny_value.ArchivalSummary()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::archival_summary::from_dafny(value.clone())),
    _ => None,
}
)
 .set_table_class_summary(match (*dafny_value.TableClassSummary()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::table_class_summary::from_dafny(value.clone())),
    _ => None,
}
)
          .build()

}
