// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::create_table::CreateTableInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableInput::CreateTableInput {
        AttributeDefinitions: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.attribute_definitions.clone().unwrap(),
    |e| crate::ddb::conversions::attribute_definition::to_dafny(e)
,
)
,
 TableName: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.table_name) .Extract(),
 KeySchema: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.key_schema.clone().unwrap(),
    |e| crate::ddb::conversions::key_schema_element::to_dafny(e)
,
)
,
 LocalSecondaryIndexes: ::std::rc::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::local_secondary_index::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 GlobalSecondaryIndexes: ::std::rc::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::global_secondary_index::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 BillingMode: ::std::rc::Rc::new(match &value.billing_mode {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::billing_mode::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 ProvisionedThroughput: ::std::rc::Rc::new(match &value.provisioned_throughput {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::provisioned_throughput::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 StreamSpecification: ::std::rc::Rc::new(match &value.stream_specification {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::stream_specification::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 SSESpecification: ::std::rc::Rc::new(match &value.sse_specification {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::sse_specification::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 Tags: ::std::rc::Rc::new(match &value.tags {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::tag::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 TableClass: ::std::rc::Rc::new(match &value.table_class {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::table_class::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableInput,
    >,
    client: aws_sdk_dynamodb::Client,
) -> aws_sdk_dynamodb::operation::create_table::builders::CreateTableFluentBuilder {
    client.create_table()
          .set_attribute_definitions(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.AttributeDefinitions(),
    |e| crate::ddb::conversions::attribute_definition::from_dafny(e.clone())
,
)
 ))
 .set_table_name(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableName()) ))
 .set_key_schema(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.KeySchema(),
    |e| crate::ddb::conversions::key_schema_element::from_dafny(e.clone())
,
)
 ))
 .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::local_secondary_index::from_dafny(e.clone())
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
                |e| crate::ddb::conversions::global_secondary_index::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_billing_mode(match &**dafny_value.BillingMode() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::billing_mode::from_dafny(value)
    ),
    _ => None,
}
)
 .set_provisioned_throughput(match (*dafny_value.ProvisionedThroughput()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::provisioned_throughput::from_dafny(value.clone())),
    _ => None,
}
)
 .set_stream_specification(match (*dafny_value.StreamSpecification()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::stream_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_sse_specification(match (*dafny_value.SSESpecification()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::sse_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_tags(match (*dafny_value.Tags()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::tag::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_table_class(match &**dafny_value.TableClass() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::table_class::from_dafny(value)
    ),
    _ => None,
}
)
}
