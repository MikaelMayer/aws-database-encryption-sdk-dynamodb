#!/bin/bash

cd $( dirname -- "${BASH_SOURCE[0]}" )

rm -rf src/primitives
cp -r ../../../../submodules/MaterialProviders/AwsCryptographyPrimitives/runtimes/rust/src/ src/primitives

rm -f src/primitives/implementation_from_dafny.rs
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::_Wrappers_Compile/crate::Wrappers/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::r#_Wrappers_Compile/crate::Wrappers/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::client/crate::primitives::client/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::conversions/crate::primitives::conversions/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::error/crate::primitives::error/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::operation/crate::primitives::operation/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::types/crate::primitives::types/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::wrapped/crate::primitives::wrapped/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::standard_library_conversions/crate::ddb::standard_library_conversions/'
find src/primitives -name '*.rs' | xargs perl -i -pe 's/crate::r#software::amazon::cryptography::primitives::internaldafny::wrapped::/crate::software::amazon::cryptography::materialproviders::internaldafny::wrapped::/'
