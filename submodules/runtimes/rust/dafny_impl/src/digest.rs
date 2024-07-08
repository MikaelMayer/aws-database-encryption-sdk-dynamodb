// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod ExternDigest {
    use crate::*;
    use aws_lc_rs::digest;

    pub struct _default {}
    impl _default {
        #[allow(non_snake_case)]
        pub fn Digest(
            digestAlgorithm: &::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::DigestAlgorithm>,
            message: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error,
                >,
            >,
        > {
            let algorithm = match digestAlgorithm {
                SHA_512 => &digest::SHA512,
                SHA_384 => &digest::SHA384,
                SHA_256 => &digest::SHA256,
            };
            let message_vec: Vec<u8> = message.iter().collect();
            let result = digest::digest(&algorithm, &message_vec);
            ::std::rc::Rc::new(Wrappers::Result::Success {
                value: result.as_ref().iter().cloned().collect(),
            })
        }
    }
}
