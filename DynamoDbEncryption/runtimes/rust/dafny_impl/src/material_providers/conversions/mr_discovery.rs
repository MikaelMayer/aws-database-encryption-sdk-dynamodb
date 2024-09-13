// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::MrDiscovery,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::material_providers::types::MrDiscovery,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery {
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery::MRDiscovery {
        region: crate::ddb::standard_library_conversions::ostring_to_dafny(&value.region) .Extract(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::material_providers::types::MrDiscovery>,
) -> ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
  crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery,
>>>{
    ::std::rc::Rc::new(match value {
        ::std::option::Option::None => crate::Wrappers::Option::None {},
        ::std::option::Option::Some(x) => crate::Wrappers::Option::Some {
            value: ::std::rc::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery,
    >,
) -> crate::material_providers::types::MrDiscovery {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery,
) -> crate::material_providers::types::MrDiscovery {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery::MRDiscovery {..} =>
            crate::material_providers::types::MrDiscovery::builder()
                .set_region(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.region()) ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::MRDiscovery,
    >>>,
) -> ::std::option::Option<crate::material_providers::types::MrDiscovery> {
    match &*dafny_value {
        crate::Wrappers::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
