pub use dkg::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod dkg {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("threshold"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("duration"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PHASE_DURATION"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PHASE_DURATION"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("THRESHOLD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("THRESHOLD"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowlist"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowlist"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBlsKeys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBlsKeys"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getJustifications"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getJustifications"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getParticipants"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getParticipants"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getResponses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getResponses"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getShares"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inPhase"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("inPhase"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("justifications"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("justifications"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("keys"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("keys"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("participants"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("participants"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("publish"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("publish"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("value"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("register"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("blsPublicKey"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("responses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("responses"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("shares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("shares"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("start"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("start"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("startBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("startBlock"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("userState"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum DKG.UserState"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DKG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct DKG<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DKG<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DKG<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DKG<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DKG<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DKG))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DKG<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DKG_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `PHASE_DURATION` (0x4ae2b849) function
        pub fn phase_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 226, 184, 73], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `THRESHOLD` (0x785ffb37) function
        pub fn threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 95, 251, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowlist` (0xa7cd52cb) function
        pub fn allowlist(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 205, 82, 203], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlsKeys` (0xa8194596) function
        pub fn get_bls_keys(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::Bytes>,
            ),
        > {
            self.0
                .method_hash([168, 25, 69, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getJustifications` (0xb0ef8179) function
        pub fn get_justifications(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([176, 239, 129, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParticipants` (0x5aa68ac0) function
        pub fn get_participants(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([90, 166, 138, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getResponses` (0xcc5ef009) function
        pub fn get_responses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([204, 94, 240, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getShares` (0xd73fe0aa) function
        pub fn get_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([215, 63, 224, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inPhase` (0x221f9511) function
        pub fn in_phase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([34, 31, 149, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `justifications` (0xcd5e3837) function
        pub fn justifications(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([205, 94, 56, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `keys` (0x670d14b2) function
        pub fn keys(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([103, 13, 20, 178], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `participants` (0x35c1d349) function
        pub fn participants(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([53, 193, 211, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `publish` (0x7fd28346) function
        pub fn publish(
            &self,
            value: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 210, 131, 70], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0x82fbdc9c) function
        pub fn register(
            &self,
            bls_public_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 251, 220, 156], bls_public_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `responses` (0x0ea65648) function
        pub fn responses(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([14, 166, 86, 72], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shares` (0xce7c2ac2) function
        pub fn shares(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([206, 124, 42, 194], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `start` (0xbe9a6555) function
        pub fn start(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 154, 101, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBlock` (0x48cd4cb1) function
        pub fn start_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 205, 76, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userState` (0x0c8f81b5) function
        pub fn user_state(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([12, 143, 129, 181], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DKG<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `PHASE_DURATION` function with signature `PHASE_DURATION()` and selector `0x4ae2b849`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PHASE_DURATION", abi = "PHASE_DURATION()")]
    pub struct PhaseDurationCall;
    ///Container type for all input parameters for the `THRESHOLD` function with signature `THRESHOLD()` and selector `0x785ffb37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "THRESHOLD", abi = "THRESHOLD()")]
    pub struct ThresholdCall;
    ///Container type for all input parameters for the `allowlist` function with signature `allowlist(address)` and selector `0xa7cd52cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "allowlist", abi = "allowlist(address)")]
    pub struct AllowlistCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getBlsKeys` function with signature `getBlsKeys()` and selector `0xa8194596`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBlsKeys", abi = "getBlsKeys()")]
    pub struct GetBlsKeysCall;
    ///Container type for all input parameters for the `getJustifications` function with signature `getJustifications()` and selector `0xb0ef8179`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getJustifications", abi = "getJustifications()")]
    pub struct GetJustificationsCall;
    ///Container type for all input parameters for the `getParticipants` function with signature `getParticipants()` and selector `0x5aa68ac0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getParticipants", abi = "getParticipants()")]
    pub struct GetParticipantsCall;
    ///Container type for all input parameters for the `getResponses` function with signature `getResponses()` and selector `0xcc5ef009`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getResponses", abi = "getResponses()")]
    pub struct GetResponsesCall;
    ///Container type for all input parameters for the `getShares` function with signature `getShares()` and selector `0xd73fe0aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getShares", abi = "getShares()")]
    pub struct GetSharesCall;
    ///Container type for all input parameters for the `inPhase` function with signature `inPhase()` and selector `0x221f9511`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "inPhase", abi = "inPhase()")]
    pub struct InPhaseCall;
    ///Container type for all input parameters for the `justifications` function with signature `justifications(address)` and selector `0xcd5e3837`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "justifications", abi = "justifications(address)")]
    pub struct JustificationsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `keys` function with signature `keys(address)` and selector `0x670d14b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "keys", abi = "keys(address)")]
    pub struct KeysCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `participants` function with signature `participants(uint256)` and selector `0x35c1d349`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "participants", abi = "participants(uint256)")]
    pub struct ParticipantsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `publish` function with signature `publish(bytes)` and selector `0x7fd28346`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "publish", abi = "publish(bytes)")]
    pub struct PublishCall {
        pub value: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `register` function with signature `register(bytes)` and selector `0x82fbdc9c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "register", abi = "register(bytes)")]
    pub struct RegisterCall {
        pub bls_public_key: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `responses` function with signature `responses(address)` and selector `0x0ea65648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "responses", abi = "responses(address)")]
    pub struct ResponsesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "shares", abi = "shares(address)")]
    pub struct SharesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `start` function with signature `start()` and selector `0xbe9a6555`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "start", abi = "start()")]
    pub struct StartCall;
    ///Container type for all input parameters for the `startBlock` function with signature `startBlock()` and selector `0x48cd4cb1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "startBlock", abi = "startBlock()")]
    pub struct StartBlockCall;
    ///Container type for all input parameters for the `userState` function with signature `userState(address)` and selector `0x0c8f81b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "userState", abi = "userState(address)")]
    pub struct UserStateCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DKGCalls {
        PhaseDuration(PhaseDurationCall),
        Threshold(ThresholdCall),
        Allowlist(AllowlistCall),
        GetBlsKeys(GetBlsKeysCall),
        GetJustifications(GetJustificationsCall),
        GetParticipants(GetParticipantsCall),
        GetResponses(GetResponsesCall),
        GetShares(GetSharesCall),
        InPhase(InPhaseCall),
        Justifications(JustificationsCall),
        Keys(KeysCall),
        Owner(OwnerCall),
        Participants(ParticipantsCall),
        Publish(PublishCall),
        Register(RegisterCall),
        Responses(ResponsesCall),
        Shares(SharesCall),
        Start(StartCall),
        StartBlock(StartBlockCall),
        UserState(UserStateCall),
    }
    impl ::ethers::core::abi::AbiDecode for DKGCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PhaseDurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PhaseDuration(decoded));
            }
            if let Ok(decoded) = <ThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Threshold(decoded));
            }
            if let Ok(decoded) = <AllowlistCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowlist(decoded));
            }
            if let Ok(decoded) = <GetBlsKeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBlsKeys(decoded));
            }
            if let Ok(decoded) =
                <GetJustificationsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetJustifications(decoded));
            }
            if let Ok(decoded) =
                <GetParticipantsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetParticipants(decoded));
            }
            if let Ok(decoded) = <GetResponsesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetResponses(decoded));
            }
            if let Ok(decoded) = <GetSharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetShares(decoded));
            }
            if let Ok(decoded) = <InPhaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InPhase(decoded));
            }
            if let Ok(decoded) =
                <JustificationsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Justifications(decoded));
            }
            if let Ok(decoded) = <KeysCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Keys(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ParticipantsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Participants(decoded));
            }
            if let Ok(decoded) = <PublishCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Publish(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <ResponsesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Responses(decoded));
            }
            if let Ok(decoded) = <SharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Shares(decoded));
            }
            if let Ok(decoded) = <StartCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Start(decoded));
            }
            if let Ok(decoded) = <StartBlockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StartBlock(decoded));
            }
            if let Ok(decoded) = <UserStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UserState(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DKGCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PhaseDuration(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Threshold(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowlist(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBlsKeys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetJustifications(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetParticipants(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetResponses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InPhase(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Justifications(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Keys(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Participants(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Publish(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Register(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Responses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Shares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Start(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StartBlock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserState(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DKGCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PhaseDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Threshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowlist(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlsKeys(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetJustifications(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParticipants(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetResponses(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::InPhase(element) => ::core::fmt::Display::fmt(element, f),
                Self::Justifications(element) => ::core::fmt::Display::fmt(element, f),
                Self::Keys(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Participants(element) => ::core::fmt::Display::fmt(element, f),
                Self::Publish(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::Responses(element) => ::core::fmt::Display::fmt(element, f),
                Self::Shares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Start(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserState(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PhaseDurationCall> for DKGCalls {
        fn from(value: PhaseDurationCall) -> Self {
            Self::PhaseDuration(value)
        }
    }
    impl ::core::convert::From<ThresholdCall> for DKGCalls {
        fn from(value: ThresholdCall) -> Self {
            Self::Threshold(value)
        }
    }
    impl ::core::convert::From<AllowlistCall> for DKGCalls {
        fn from(value: AllowlistCall) -> Self {
            Self::Allowlist(value)
        }
    }
    impl ::core::convert::From<GetBlsKeysCall> for DKGCalls {
        fn from(value: GetBlsKeysCall) -> Self {
            Self::GetBlsKeys(value)
        }
    }
    impl ::core::convert::From<GetJustificationsCall> for DKGCalls {
        fn from(value: GetJustificationsCall) -> Self {
            Self::GetJustifications(value)
        }
    }
    impl ::core::convert::From<GetParticipantsCall> for DKGCalls {
        fn from(value: GetParticipantsCall) -> Self {
            Self::GetParticipants(value)
        }
    }
    impl ::core::convert::From<GetResponsesCall> for DKGCalls {
        fn from(value: GetResponsesCall) -> Self {
            Self::GetResponses(value)
        }
    }
    impl ::core::convert::From<GetSharesCall> for DKGCalls {
        fn from(value: GetSharesCall) -> Self {
            Self::GetShares(value)
        }
    }
    impl ::core::convert::From<InPhaseCall> for DKGCalls {
        fn from(value: InPhaseCall) -> Self {
            Self::InPhase(value)
        }
    }
    impl ::core::convert::From<JustificationsCall> for DKGCalls {
        fn from(value: JustificationsCall) -> Self {
            Self::Justifications(value)
        }
    }
    impl ::core::convert::From<KeysCall> for DKGCalls {
        fn from(value: KeysCall) -> Self {
            Self::Keys(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DKGCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ParticipantsCall> for DKGCalls {
        fn from(value: ParticipantsCall) -> Self {
            Self::Participants(value)
        }
    }
    impl ::core::convert::From<PublishCall> for DKGCalls {
        fn from(value: PublishCall) -> Self {
            Self::Publish(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for DKGCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<ResponsesCall> for DKGCalls {
        fn from(value: ResponsesCall) -> Self {
            Self::Responses(value)
        }
    }
    impl ::core::convert::From<SharesCall> for DKGCalls {
        fn from(value: SharesCall) -> Self {
            Self::Shares(value)
        }
    }
    impl ::core::convert::From<StartCall> for DKGCalls {
        fn from(value: StartCall) -> Self {
            Self::Start(value)
        }
    }
    impl ::core::convert::From<StartBlockCall> for DKGCalls {
        fn from(value: StartBlockCall) -> Self {
            Self::StartBlock(value)
        }
    }
    impl ::core::convert::From<UserStateCall> for DKGCalls {
        fn from(value: UserStateCall) -> Self {
            Self::UserState(value)
        }
    }
    ///Container type for all return fields from the `PHASE_DURATION` function with signature `PHASE_DURATION()` and selector `0x4ae2b849`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PhaseDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `THRESHOLD` function with signature `THRESHOLD()` and selector `0x785ffb37`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBlsKeys` function with signature `getBlsKeys()` and selector `0xa8194596`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetBlsKeysReturn(
        pub ::ethers::core::types::U256,
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
    );
    ///Container type for all return fields from the `getJustifications` function with signature `getJustifications()` and selector `0xb0ef8179`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetJustificationsReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
    ///Container type for all return fields from the `getParticipants` function with signature `getParticipants()` and selector `0x5aa68ac0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetParticipantsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getResponses` function with signature `getResponses()` and selector `0xcc5ef009`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetResponsesReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
    ///Container type for all return fields from the `getShares` function with signature `getShares()` and selector `0xd73fe0aa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSharesReturn(pub ::std::vec::Vec<::ethers::core::types::Bytes>);
    ///Container type for all return fields from the `inPhase` function with signature `inPhase()` and selector `0x221f9511`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InPhaseReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `justifications` function with signature `justifications(address)` and selector `0xcd5e3837`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct JustificationsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `keys` function with signature `keys(address)` and selector `0x670d14b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct KeysReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `participants` function with signature `participants(uint256)` and selector `0x35c1d349`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ParticipantsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `responses` function with signature `responses(address)` and selector `0x0ea65648`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ResponsesReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SharesReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `startBlock` function with signature `startBlock()` and selector `0x48cd4cb1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StartBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userState` function with signature `userState(address)` and selector `0x0c8f81b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UserStateReturn(pub u8);
}
