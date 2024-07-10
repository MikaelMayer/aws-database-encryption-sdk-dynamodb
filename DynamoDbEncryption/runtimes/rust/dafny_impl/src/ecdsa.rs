// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod Signature {
    pub mod ECDSA {
        use crate::*;
    pub use crate::software::amazon::cryptography::materialproviders::internaldafny::types::ECDSA::*;

        pub fn ExternKeyGen(
            _self: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDSASignatureAlgorithm>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::std::rc::Rc<Signature::SignatureKeyPair>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("Signature::ExternKeyGen not implemented");
        }
        pub fn Sign(
            _self: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDSASignatureAlgorithm>,
            _key: &::dafny_runtime::Sequence<u8>,
            _msg: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("Signature::Sign not implemented");
        }
        /*
        pub fn ECDSAVerify(config: &::std::rc::Rc<AwsCryptographyPrimitivesOperations::Config>, input: &::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::ECDSAVerifyInput>) -> ::std::rc::Rc<Wrappers::Result<bool, ::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::Error>>> {
          let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<Wrappers::Result<bool, ::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::Error>>>>::new();
          output = ::dafny_runtime::MaybePlacebo::from(crate::_externs::Signature_dECDSA::Verify(input.signatureAlgorithm(); input.verificationKey(), input.message(), input.signature()));
          return output.read(); */
        pub fn Verify(
            _self: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDSASignatureAlgorithm>,
            _key: &::dafny_runtime::Sequence<u8>,
            _msg: &::dafny_runtime::Sequence<u8>,
            _sig: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                bool,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("Signature::Verify not implemented");
        }
    }
}
