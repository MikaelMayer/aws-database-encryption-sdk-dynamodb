// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::Discovery,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::material_providers::types::Discovery,
) -> crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery {
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery::Discovery {

    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::material_providers::types::Discovery>,
) -> ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
  crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery,
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
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery,
    >,
) -> crate::material_providers::types::Discovery {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery,
) -> crate::material_providers::types::Discovery {
    match dafny_value {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery::Discovery {..} =>
            crate::material_providers::types::Discovery::builder()

                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::Discovery,
    >>>,
) -> ::std::option::Option<crate::material_providers::types::Discovery> {
    match &*dafny_value {
        crate::Wrappers::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
