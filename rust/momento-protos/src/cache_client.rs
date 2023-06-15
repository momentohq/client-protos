#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(enumeration = "ECacheResult", tag = "1")]
    pub result: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub cache_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub cache_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetResponse {
    #[prost(enumeration = "ECacheResult", tag = "1")]
    pub result: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIfNotExistsRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub cache_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIfNotExistsResponse {
    #[prost(oneof = "set_if_not_exists_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<set_if_not_exists_response::Result>,
}
/// Nested message and enum types in `_SetIfNotExistsResponse`.
pub mod set_if_not_exists_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stored {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotStored {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Stored(Stored),
        #[prost(message, tag = "2")]
        NotStored(NotStored),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysExistRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub cache_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysExistResponse {
    #[prost(bool, repeated, tag = "1")]
    pub exists: ::prost::alloc::vec::Vec<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
    /// Amount to add to the stored value.
    /// If this key doesn't currently exist, it's created with this value (encoded as a base 10 string)
    #[prost(int64, tag = "2")]
    pub amount: i64,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementResponse {
    /// The value stored after the increment operation.
    #[prost(int64, tag = "1")]
    pub value: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTtlRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "update_ttl_request::UpdateTtl", tags = "2, 3, 4")]
    pub update_ttl: ::core::option::Option<update_ttl_request::UpdateTtl>,
}
/// Nested message and enum types in `_UpdateTtlRequest`.
pub mod update_ttl_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UpdateTtl {
        /// Sets the ttl to this value only if it is an increase compared to the existing ttl
        #[prost(uint64, tag = "2")]
        IncreaseToMilliseconds(u64),
        /// Sets the ttl to this value only if it is a decrease compared to the existing ttl
        #[prost(uint64, tag = "3")]
        DecreaseToMilliseconds(u64),
        /// Sets the ttl to this value unconditionally
        #[prost(uint64, tag = "4")]
        OverwriteToMilliseconds(u64),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTtlResponse {
    #[prost(oneof = "update_ttl_response::Result", tags = "1, 2, 3")]
    pub result: ::core::option::Option<update_ttl_response::Result>,
}
/// Nested message and enum types in `_UpdateTtlResponse`.
pub mod update_ttl_response {
    /// Indicates that the ttl was applied.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Set {}
    /// Indicates that the ttl was not applied due to a failed condition.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotSet {}
    /// Indicates that the key did not exist.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Set(Set),
        #[prost(message, tag = "2")]
        NotSet(NotSet),
        #[prost(message, tag = "3")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGetTtlRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGetTtlResponse {
    #[prost(oneof = "item_get_ttl_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<item_get_ttl_response::Result>,
}
/// Nested message and enum types in `_ItemGetTtlResponse`.
pub mod item_get_ttl_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint64, tag = "1")]
        pub remaining_ttl_millis: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGetTypeRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub cache_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGetTypeResponse {
    #[prost(oneof = "item_get_type_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<item_get_type_response::Result>,
}
/// Nested message and enum types in `_ItemGetTypeResponse`.
pub mod item_get_type_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(enumeration = "ItemType", tag = "1")]
        pub item_type: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ItemType {
        Scalar = 0,
        Dictionary = 1,
        Set = 2,
        List = 3,
        SortedSet = 4,
    }
    impl ItemType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ItemType::Scalar => "SCALAR",
                ItemType::Dictionary => "DICTIONARY",
                ItemType::Set => "SET",
                ItemType::List => "LIST",
                ItemType::SortedSet => "SORTED_SET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SCALAR" => Some(Self::Scalar),
                "DICTIONARY" => Some(Self::Dictionary),
                "SET" => Some(Self::Set),
                "LIST" => Some(Self::List),
                "SORTED_SET" => Some(Self::SortedSet),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryGetRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryGetResponse {
    #[prost(oneof = "dictionary_get_response::Dictionary", tags = "1, 2")]
    pub dictionary: ::core::option::Option<dictionary_get_response::Dictionary>,
}
/// Nested message and enum types in `_DictionaryGetResponse`.
pub mod dictionary_get_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DictionaryGetResponsePart {
        #[prost(enumeration = "super::ECacheResult", tag = "1")]
        pub result: i32,
        #[prost(bytes = "vec", tag = "2")]
        pub cache_body: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(message, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<DictionaryGetResponsePart>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dictionary {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryFetchRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryFieldValuePair {
    #[prost(bytes = "vec", tag = "1")]
    pub field: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryFetchResponse {
    #[prost(oneof = "dictionary_fetch_response::Dictionary", tags = "1, 2")]
    pub dictionary: ::core::option::Option<dictionary_fetch_response::Dictionary>,
}
/// Nested message and enum types in `_DictionaryFetchResponse`.
pub mod dictionary_fetch_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(message, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<super::DictionaryFieldValuePair>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dictionary {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionarySetRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<DictionaryFieldValuePair>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionarySetResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryIncrementRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub field: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub amount: i64,
    #[prost(uint64, tag = "4")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "5")]
    pub refresh_ttl: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryIncrementResponse {
    #[prost(int64, tag = "1")]
    pub value: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryDeleteRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "dictionary_delete_request::Delete", tags = "2, 3")]
    pub delete: ::core::option::Option<dictionary_delete_request::Delete>,
}
/// Nested message and enum types in `_DictionaryDeleteRequest`.
pub mod dictionary_delete_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Some {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub fields: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Delete {
        #[prost(message, tag = "2")]
        Some(Some),
        #[prost(message, tag = "3")]
        All(All),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryDeleteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryLengthRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub dictionary_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictionaryLengthResponse {
    #[prost(oneof = "dictionary_length_response::Dictionary", tags = "1, 2")]
    pub dictionary: ::core::option::Option<dictionary_length_response::Dictionary>,
}
/// Nested message and enum types in `_DictionaryLengthResponse`.
pub mod dictionary_length_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dictionary {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFetchRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFetchResponse {
    #[prost(oneof = "set_fetch_response::Set", tags = "1, 2")]
    pub set: ::core::option::Option<set_fetch_response::Set>,
}
/// Nested message and enum types in `_SetFetchResponse`.
pub mod set_fetch_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Set {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUnionRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUnionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDifferenceRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "set_difference_request::Difference", tags = "2, 3")]
    pub difference: ::core::option::Option<set_difference_request::Difference>,
}
/// Nested message and enum types in `_SetDifferenceRequest`.
pub mod set_difference_request {
    /// cache = request - stored
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Minuend {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    /// cache = stored - request
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subtrahend {
        #[prost(oneof = "subtrahend::SubtrahendSet", tags = "1, 2")]
        pub subtrahend_set: ::core::option::Option<subtrahend::SubtrahendSet>,
    }
    /// Nested message and enum types in `_Subtrahend`.
    pub mod subtrahend {
        /// Subtract a set of elements
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Set {
            #[prost(bytes = "vec", repeated, tag = "1")]
            pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        }
        /// Subtract the set's identity (itself) from itself - which deletes it.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Identity {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SubtrahendSet {
            #[prost(message, tag = "1")]
            Set(Set),
            #[prost(message, tag = "2")]
            Identity(Identity),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Difference {
        #[prost(message, tag = "2")]
        Minuend(Minuend),
        #[prost(message, tag = "3")]
        Subtrahend(Subtrahend),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDifferenceResponse {
    #[prost(oneof = "set_difference_response::Set", tags = "1, 2")]
    pub set: ::core::option::Option<set_difference_response::Set>,
}
/// Nested message and enum types in `_SetDifferenceResponse`.
pub mod set_difference_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Set {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContainsRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetContainsResponse {
    #[prost(oneof = "set_contains_response::Set", tags = "1, 2")]
    pub set: ::core::option::Option<set_contains_response::Set>,
}
/// Nested message and enum types in `_SetContainsResponse`.
pub mod set_contains_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        /// This will be the same length as the elements passed in the request.
        /// It represents whether each element is a member of the set, with indices corresponding to the request elements.
        #[prost(bool, repeated, tag = "1")]
        pub contains: ::prost::alloc::vec::Vec<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Set {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLengthRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLengthResponse {
    #[prost(oneof = "set_length_response::Set", tags = "1, 2")]
    pub set: ::core::option::Option<set_length_response::Set>,
}
/// Nested message and enum types in `_SetLengthResponse`.
pub mod set_length_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Set {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConcatenateFrontRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
    /// ensure total length <= this; remove excess from back of list
    #[prost(uint32, tag = "5")]
    pub truncate_back_to_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConcatenateFrontResponse {
    /// length of the list after the concatenation
    #[prost(uint32, tag = "1")]
    pub list_length: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConcatenateBackRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
    /// ensure total length <= this; remove excess from front of list
    #[prost(uint32, tag = "5")]
    pub truncate_front_to_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConcatenateBackResponse {
    /// length of the list after the concatenation
    #[prost(uint32, tag = "1")]
    pub list_length: u32,
}
/// stored = request + stored
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPushFrontRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
    /// ensure total length <= this; remove excess from back of list
    #[prost(uint32, tag = "5")]
    pub truncate_back_to_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPushFrontResponse {
    /// length of the list after the push
    #[prost(uint32, tag = "1")]
    pub list_length: u32,
}
/// stored = stored + request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPushBackRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
    /// ensure total length <= this; remove excess from front of list
    #[prost(uint32, tag = "5")]
    pub truncate_front_to_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPushBackResponse {
    /// length of the list after the push
    #[prost(uint32, tag = "1")]
    pub list_length: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPopFrontRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPopFrontResponse {
    #[prost(oneof = "list_pop_front_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_pop_front_response::List>,
}
/// Nested message and enum types in `_ListPopFrontResponse`.
pub mod list_pop_front_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(bytes = "vec", tag = "1")]
        pub front: ::prost::alloc::vec::Vec<u8>,
        /// length of the list after the pop
        #[prost(uint32, tag = "2")]
        pub list_length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPopBackRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPopBackResponse {
    #[prost(oneof = "list_pop_back_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_pop_back_response::List>,
}
/// Nested message and enum types in `_ListPopBackResponse`.
pub mod list_pop_back_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(bytes = "vec", tag = "1")]
        pub back: ::prost::alloc::vec::Vec<u8>,
        /// length of the list after the pop
        #[prost(uint32, tag = "2")]
        pub list_length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRange {
    #[prost(uint32, tag = "1")]
    pub begin_index: u32,
    #[prost(uint32, tag = "2")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEraseRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "list_erase_request::Erase", tags = "2, 3")]
    pub erase: ::core::option::Option<list_erase_request::Erase>,
}
/// Nested message and enum types in `_ListEraseRequest`.
pub mod list_erase_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListRanges {
        #[prost(message, repeated, tag = "1")]
        pub ranges: ::prost::alloc::vec::Vec<super::ListRange>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Erase {
        #[prost(message, tag = "2")]
        Some(ListRanges),
        #[prost(message, tag = "3")]
        All(All),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEraseResponse {
    #[prost(oneof = "list_erase_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_erase_response::List>,
}
/// Nested message and enum types in `_ListEraseResponse`.
pub mod list_erase_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub list_length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoveRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "list_remove_request::Remove", tags = "2")]
    pub remove: ::core::option::Option<list_remove_request::Remove>,
}
/// Nested message and enum types in `_ListRemoveRequest`.
pub mod list_remove_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Remove {
        /// Remove all appearances in the list where the element is this value
        #[prost(bytes, tag = "2")]
        AllElementsWithValue(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoveResponse {
    #[prost(oneof = "list_remove_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_remove_response::List>,
}
/// Nested message and enum types in `_ListRemoveResponse`.
pub mod list_remove_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub list_length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unbounded {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFetchRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    /// Inclusive.
    /// If unbounded, 0 (start of list) by default
    /// A negative index counts from the end of the list
    #[prost(oneof = "list_fetch_request::StartIndex", tags = "2, 3")]
    pub start_index: ::core::option::Option<list_fetch_request::StartIndex>,
    /// Exclusive.
    /// If unbounded, this effectively means list.length()
    /// If end_index is > the number of elements to return, return as much as you can
    /// A negative index counts from the end of the list
    #[prost(oneof = "list_fetch_request::EndIndex", tags = "4, 5")]
    pub end_index: ::core::option::Option<list_fetch_request::EndIndex>,
}
/// Nested message and enum types in `_ListFetchRequest`.
pub mod list_fetch_request {
    /// Inclusive.
    /// If unbounded, 0 (start of list) by default
    /// A negative index counts from the end of the list
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartIndex {
        #[prost(message, tag = "2")]
        UnboundedStart(super::Unbounded),
        #[prost(sint32, tag = "3")]
        InclusiveStart(i32),
    }
    /// Exclusive.
    /// If unbounded, this effectively means list.length()
    /// If end_index is > the number of elements to return, return as much as you can
    /// A negative index counts from the end of the list
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EndIndex {
        #[prost(message, tag = "4")]
        UnboundedEnd(super::Unbounded),
        #[prost(sint32, tag = "5")]
        ExclusiveEnd(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRetainRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "6")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "7")]
    pub refresh_ttl: bool,
    #[prost(oneof = "list_retain_request::StartIndex", tags = "2, 3")]
    pub start_index: ::core::option::Option<list_retain_request::StartIndex>,
    #[prost(oneof = "list_retain_request::EndIndex", tags = "4, 5")]
    pub end_index: ::core::option::Option<list_retain_request::EndIndex>,
}
/// Nested message and enum types in `_ListRetainRequest`.
pub mod list_retain_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartIndex {
        #[prost(message, tag = "2")]
        UnboundedStart(super::Unbounded),
        #[prost(sint32, tag = "3")]
        InclusiveStart(i32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EndIndex {
        #[prost(message, tag = "4")]
        UnboundedEnd(super::Unbounded),
        #[prost(sint32, tag = "5")]
        ExclusiveEnd(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRetainResponse {
    #[prost(oneof = "list_retain_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_retain_response::List>,
}
/// Nested message and enum types in `_ListRetainResponse`.
pub mod list_retain_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub list_length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFetchResponse {
    #[prost(oneof = "list_fetch_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_fetch_response::List>,
}
/// Nested message and enum types in `_ListFetchResponse`.
pub mod list_fetch_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLengthRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub list_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLengthResponse {
    #[prost(oneof = "list_length_response::List", tags = "1, 2")]
    pub list: ::core::option::Option<list_length_response::List>,
}
/// Nested message and enum types in `_ListLengthResponse`.
pub mod list_length_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum List {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetElement {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "2")]
    pub score: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetPutRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub elements: ::prost::alloc::vec::Vec<SortedSetElement>,
    #[prost(uint64, tag = "3")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "4")]
    pub refresh_ttl: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetPutResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetFetchRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "sorted_set_fetch_request::Order", tag = "2")]
    pub order: i32,
    #[prost(bool, tag = "3")]
    pub with_scores: bool,
    #[prost(oneof = "sorted_set_fetch_request::Range", tags = "4, 5")]
    pub range: ::core::option::Option<sorted_set_fetch_request::Range>,
}
/// Nested message and enum types in `_SortedSetFetchRequest`.
pub mod sorted_set_fetch_request {
    /// Start and end are zero-based indexes, with 0 being the first element.
    /// A negative index indicates offsets from the end of the sorted set, with
    /// -1 being the last element.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ByIndex {
        /// Start is inclusive.
        /// Unbounded is treated as 0.
        #[prost(oneof = "by_index::Start", tags = "1, 2")]
        pub start: ::core::option::Option<by_index::Start>,
        /// End is exclusive.
        /// Unbounded is treated as the number of elements in the sorted set.
        #[prost(oneof = "by_index::End", tags = "3, 4")]
        pub end: ::core::option::Option<by_index::End>,
    }
    /// Nested message and enum types in `_ByIndex`.
    pub mod by_index {
        /// Start is inclusive.
        /// Unbounded is treated as 0.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Start {
            #[prost(message, tag = "1")]
            UnboundedStart(super::super::Unbounded),
            #[prost(sint32, tag = "2")]
            InclusiveStartIndex(i32),
        }
        /// End is exclusive.
        /// Unbounded is treated as the number of elements in the sorted set.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum End {
            #[prost(message, tag = "3")]
            UnboundedEnd(super::super::Unbounded),
            #[prost(sint32, tag = "4")]
            ExclusiveEndIndex(i32),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ByScore {
        /// Offset and count are used to only get a range of the matching elements,
        /// similar to "SELECT LIMIT offset, count" in SQL.
        /// A negative count returns all elements from the offset.
        /// Use (0, -1) to return all matching elements.
        #[prost(uint32, tag = "5")]
        pub offset: u32,
        #[prost(sint32, tag = "6")]
        pub count: i32,
        #[prost(oneof = "by_score::Min", tags = "1, 2")]
        pub min: ::core::option::Option<by_score::Min>,
        #[prost(oneof = "by_score::Max", tags = "3, 4")]
        pub max: ::core::option::Option<by_score::Max>,
    }
    /// Nested message and enum types in `_ByScore`.
    pub mod by_score {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Score {
            #[prost(double, tag = "1")]
            pub score: f64,
            #[prost(bool, tag = "2")]
            pub exclusive: bool,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Min {
            #[prost(message, tag = "1")]
            UnboundedMin(super::super::Unbounded),
            #[prost(message, tag = "2")]
            MinScore(Score),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Max {
            #[prost(message, tag = "3")]
            UnboundedMax(super::super::Unbounded),
            #[prost(message, tag = "4")]
            MaxScore(Score),
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Order {
        Ascending = 0,
        Descending = 1,
    }
    impl Order {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Order::Ascending => "ASCENDING",
                Order::Descending => "DESCENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ASCENDING" => Some(Self::Ascending),
                "DESCENDING" => Some(Self::Descending),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Range {
        #[prost(message, tag = "4")]
        ByIndex(ByIndex),
        #[prost(message, tag = "5")]
        ByScore(ByScore),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetFetchResponse {
    #[prost(oneof = "sorted_set_fetch_response::SortedSet", tags = "1, 2")]
    pub sorted_set: ::core::option::Option<sorted_set_fetch_response::SortedSet>,
}
/// Nested message and enum types in `_SortedSetFetchResponse`.
pub mod sorted_set_fetch_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(oneof = "found::Elements", tags = "1, 2")]
        pub elements: ::core::option::Option<found::Elements>,
    }
    /// Nested message and enum types in `_Found`.
    pub mod found {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ValuesWithScores {
            #[prost(message, repeated, tag = "1")]
            pub elements: ::prost::alloc::vec::Vec<super::super::SortedSetElement>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Values {
            #[prost(bytes = "vec", repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Elements {
            #[prost(message, tag = "1")]
            ValuesWithScores(ValuesWithScores),
            #[prost(message, tag = "2")]
            Values(Values),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortedSet {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetGetScoreRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetGetScoreResponse {
    #[prost(oneof = "sorted_set_get_score_response::SortedSet", tags = "1, 2")]
    pub sorted_set: ::core::option::Option<sorted_set_get_score_response::SortedSet>,
}
/// Nested message and enum types in `_SortedSetGetScoreResponse`.
pub mod sorted_set_get_score_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortedSetGetScoreResponsePart {
        #[prost(enumeration = "super::ECacheResult", tag = "1")]
        pub result: i32,
        #[prost(double, tag = "2")]
        pub score: f64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortedSetFound {
        #[prost(message, repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<SortedSetGetScoreResponsePart>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortedSetMissing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortedSet {
        #[prost(message, tag = "1")]
        Found(SortedSetFound),
        #[prost(message, tag = "2")]
        Missing(SortedSetMissing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetRemoveRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "sorted_set_remove_request::RemoveElements", tags = "2, 3")]
    pub remove_elements: ::core::option::Option<
        sorted_set_remove_request::RemoveElements,
    >,
}
/// Nested message and enum types in `_SortedSetRemoveRequest`.
pub mod sorted_set_remove_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct All {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Some {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RemoveElements {
        #[prost(message, tag = "2")]
        All(All),
        #[prost(message, tag = "3")]
        Some(Some),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetRemoveResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetIncrementRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(uint64, tag = "4")]
    pub ttl_milliseconds: u64,
    #[prost(bool, tag = "5")]
    pub refresh_ttl: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetIncrementResponse {
    /// The updated score stored after the increment operation.
    #[prost(double, tag = "1")]
    pub score: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetGetRankRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The order in which sorted set will be sorted to determine the rank.
    ///
    /// When Order.ASCENDING is specified, will return the rank of the value
    /// when sorted set scores are ordered from low to high. This is the default
    /// when no Order is specified.
    ///
    /// When Order.DESCENDING is specified, will return the rank of the value
    /// when sorted set scores are ordered from high to low.
    #[prost(enumeration = "sorted_set_get_rank_request::Order", tag = "3")]
    pub order: i32,
}
/// Nested message and enum types in `_SortedSetGetRankRequest`.
pub mod sorted_set_get_rank_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Order {
        Ascending = 0,
        Descending = 1,
    }
    impl Order {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Order::Ascending => "ASCENDING",
                Order::Descending => "DESCENDING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ASCENDING" => Some(Self::Ascending),
                "DESCENDING" => Some(Self::Descending),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetGetRankResponse {
    #[prost(oneof = "sorted_set_get_rank_response::Rank", tags = "1, 2")]
    pub rank: ::core::option::Option<sorted_set_get_rank_response::Rank>,
}
/// Nested message and enum types in `_SortedSetGetRankResponse`.
pub mod sorted_set_get_rank_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RankResponsePart {
        #[prost(enumeration = "super::ECacheResult", tag = "1")]
        pub result: i32,
        /// Rank is 0-based i.e. when sort order is descending the rank of the
        /// value with the highest score will be 0. Similarly for ascending order,
        /// value with the lowest score will have rank 0.
        #[prost(uint64, tag = "2")]
        pub rank: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SortedSetMissing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rank {
        #[prost(message, tag = "1")]
        ElementRank(RankResponsePart),
        #[prost(message, tag = "2")]
        Missing(SortedSetMissing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetLengthRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetLengthResponse {
    #[prost(oneof = "sorted_set_length_response::SortedSet", tags = "1, 2")]
    pub sorted_set: ::core::option::Option<sorted_set_length_response::SortedSet>,
}
/// Nested message and enum types in `_SortedSetLengthResponse`.
pub mod sorted_set_length_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortedSet {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetLengthByScoreRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub set_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "sorted_set_length_by_score_request::Min", tags = "2, 3, 4")]
    pub min: ::core::option::Option<sorted_set_length_by_score_request::Min>,
    #[prost(oneof = "sorted_set_length_by_score_request::Max", tags = "5, 6, 7")]
    pub max: ::core::option::Option<sorted_set_length_by_score_request::Max>,
}
/// Nested message and enum types in `_SortedSetLengthByScoreRequest`.
pub mod sorted_set_length_by_score_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Min {
        #[prost(double, tag = "2")]
        InclusiveMin(f64),
        #[prost(double, tag = "3")]
        ExclusiveMin(f64),
        #[prost(message, tag = "4")]
        UnboundedMin(super::Unbounded),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Max {
        #[prost(double, tag = "5")]
        InclusiveMax(f64),
        #[prost(double, tag = "6")]
        ExclusiveMax(f64),
        #[prost(message, tag = "7")]
        UnboundedMax(super::Unbounded),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortedSetLengthByScoreResponse {
    #[prost(oneof = "sorted_set_length_by_score_response::SortedSet", tags = "1, 2")]
    pub sorted_set: ::core::option::Option<
        sorted_set_length_by_score_response::SortedSet,
    >,
}
/// Nested message and enum types in `_SortedSetLengthByScoreResponse`.
pub mod sorted_set_length_by_score_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Found {
        #[prost(uint32, tag = "1")]
        pub length: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Missing {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortedSet {
        #[prost(message, tag = "1")]
        Found(Found),
        #[prost(message, tag = "2")]
        Missing(Missing),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECacheResult {
    Invalid = 0,
    Ok = 1,
    Hit = 2,
    Miss = 3,
}
impl ECacheResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ECacheResult::Invalid => "Invalid",
            ECacheResult::Ok => "Ok",
            ECacheResult::Hit => "Hit",
            ECacheResult::Miss => "Miss",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid" => Some(Self::Invalid),
            "Ok" => Some(Self::Ok),
            "Hit" => Some(Self::Hit),
            "Miss" => Some(Self::Miss),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod scs_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ScsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ScsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ScsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ScsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ScsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cache_client.Scs/Get");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cache_client.Scs", "Get"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRequest>,
        ) -> std::result::Result<tonic::Response<super::SetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cache_client.Scs/Set");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cache_client.Scs", "Set"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_if_not_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::SetIfNotExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetIfNotExistsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetIfNotExists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SetIfNotExists"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cache_client.Scs/Delete");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cache_client.Scs", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn keys_exist(
            &mut self,
            request: impl tonic::IntoRequest<super::KeysExistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeysExistResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/KeysExist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "KeysExist"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn increment(
            &mut self,
            request: impl tonic::IntoRequest<super::IncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/Increment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "Increment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_ttl(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTtlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTtlResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/UpdateTtl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "UpdateTtl"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn item_get_ttl(
            &mut self,
            request: impl tonic::IntoRequest<super::ItemGetTtlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ItemGetTtlResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ItemGetTtl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ItemGetTtl"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn item_get_type(
            &mut self,
            request: impl tonic::IntoRequest<super::ItemGetTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ItemGetTypeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ItemGetType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ItemGetType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_get(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionaryGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryGetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionaryGet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionaryGet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_fetch(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionaryFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryFetchResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionaryFetch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionaryFetch"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionarySetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionarySetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionarySet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionarySet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_increment(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionaryIncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryIncrementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionaryIncrement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionaryIncrement"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionaryDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryDeleteResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionaryDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionaryDelete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn dictionary_length(
            &mut self,
            request: impl tonic::IntoRequest<super::DictionaryLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryLengthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/DictionaryLength",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "DictionaryLength"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_fetch(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetFetchResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetFetch",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cache_client.Scs", "SetFetch"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_union(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUnionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUnionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetUnion",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("cache_client.Scs", "SetUnion"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_difference(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDifferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDifferenceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetDifference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SetDifference"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_contains(
            &mut self,
            request: impl tonic::IntoRequest<super::SetContainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetContainsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetContains",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SetContains"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_length(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetLengthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SetLength",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SetLength"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_push_front(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPushFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPushFrontResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListPushFront",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListPushFront"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_push_back(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPushBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPushBackResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListPushBack",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListPushBack"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pop_front(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPopFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPopFrontResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListPopFront",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListPopFront"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pop_back(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPopBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPopBackResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListPopBack",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListPopBack"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_erase(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEraseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEraseResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListErase",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListErase"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_remove(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRemoveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRemoveResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListRemove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListRemove"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_fetch(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFetchResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListFetch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListFetch"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_length(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLengthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListLength",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListLength"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_concatenate_front(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConcatenateFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConcatenateFrontResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListConcatenateFront",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListConcatenateFront"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_concatenate_back(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConcatenateBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConcatenateBackResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListConcatenateBack",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListConcatenateBack"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_retain(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRetainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRetainResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/ListRetain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "ListRetain"));
            self.inner.unary(req, path, codec).await
        }
        /// Add or Updates new element with its score to the Sorted Set.
        /// If sorted set doesn't exist, a new one is created with the specified
        /// element and its associated score.
        /// If an element exists, then its associate score gets overridden with the one
        /// provided in this operation.
        pub async fn sorted_set_put(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetPutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetPutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetPut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetPut"));
            self.inner.unary(req, path, codec).await
        }
        /// Fetches a subset of elements in the sorted set.
        pub async fn sorted_set_fetch(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetFetchResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetFetch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetFetch"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets the specified element and its associated score if it exists in the
        /// sorted set.
        pub async fn sorted_set_get_score(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetGetScoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetGetScoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetGetScore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetGetScore"));
            self.inner.unary(req, path, codec).await
        }
        /// Removes specified elements and their associated scores
        pub async fn sorted_set_remove(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetRemoveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetRemoveResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetRemove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetRemove"));
            self.inner.unary(req, path, codec).await
        }
        /// Changes the score associated with the element by specified amount.
        /// If the provided amount is negative, then the score associated with the
        /// element is decremented.
        /// If the element that needs to be incremented isn't present in the sorted
        /// set, it is added with specified number as the score.
        /// If the set itself doesn't exist then a new one with specified element and
        /// score is created.
        pub async fn sorted_set_increment(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetIncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetIncrementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetIncrement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetIncrement"));
            self.inner.unary(req, path, codec).await
        }
        /// Gives the rank of an element.
        pub async fn sorted_set_get_rank(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetGetRankRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetGetRankResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetGetRank",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetGetRank"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns length of the sorted set
        pub async fn sorted_set_length(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetLengthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetLength",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetLength"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns number of elements in the sorted set between a given min and max score
        pub async fn sorted_set_length_by_score(
            &mut self,
            request: impl tonic::IntoRequest<super::SortedSetLengthByScoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetLengthByScoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cache_client.Scs/SortedSetLengthByScore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cache_client.Scs", "SortedSetLengthByScore"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scs_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ScsServer.
    #[async_trait]
    pub trait Scs: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status>;
        async fn set(
            &self,
            request: tonic::Request<super::SetRequest>,
        ) -> std::result::Result<tonic::Response<super::SetResponse>, tonic::Status>;
        async fn set_if_not_exists(
            &self,
            request: tonic::Request<super::SetIfNotExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetIfNotExistsResponse>,
            tonic::Status,
        >;
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> std::result::Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
        async fn keys_exist(
            &self,
            request: tonic::Request<super::KeysExistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::KeysExistResponse>,
            tonic::Status,
        >;
        async fn increment(
            &self,
            request: tonic::Request<super::IncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementResponse>,
            tonic::Status,
        >;
        async fn update_ttl(
            &self,
            request: tonic::Request<super::UpdateTtlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTtlResponse>,
            tonic::Status,
        >;
        async fn item_get_ttl(
            &self,
            request: tonic::Request<super::ItemGetTtlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ItemGetTtlResponse>,
            tonic::Status,
        >;
        async fn item_get_type(
            &self,
            request: tonic::Request<super::ItemGetTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ItemGetTypeResponse>,
            tonic::Status,
        >;
        async fn dictionary_get(
            &self,
            request: tonic::Request<super::DictionaryGetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryGetResponse>,
            tonic::Status,
        >;
        async fn dictionary_fetch(
            &self,
            request: tonic::Request<super::DictionaryFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryFetchResponse>,
            tonic::Status,
        >;
        async fn dictionary_set(
            &self,
            request: tonic::Request<super::DictionarySetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionarySetResponse>,
            tonic::Status,
        >;
        async fn dictionary_increment(
            &self,
            request: tonic::Request<super::DictionaryIncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryIncrementResponse>,
            tonic::Status,
        >;
        async fn dictionary_delete(
            &self,
            request: tonic::Request<super::DictionaryDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryDeleteResponse>,
            tonic::Status,
        >;
        async fn dictionary_length(
            &self,
            request: tonic::Request<super::DictionaryLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DictionaryLengthResponse>,
            tonic::Status,
        >;
        async fn set_fetch(
            &self,
            request: tonic::Request<super::SetFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetFetchResponse>,
            tonic::Status,
        >;
        async fn set_union(
            &self,
            request: tonic::Request<super::SetUnionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetUnionResponse>,
            tonic::Status,
        >;
        async fn set_difference(
            &self,
            request: tonic::Request<super::SetDifferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetDifferenceResponse>,
            tonic::Status,
        >;
        async fn set_contains(
            &self,
            request: tonic::Request<super::SetContainsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetContainsResponse>,
            tonic::Status,
        >;
        async fn set_length(
            &self,
            request: tonic::Request<super::SetLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetLengthResponse>,
            tonic::Status,
        >;
        async fn list_push_front(
            &self,
            request: tonic::Request<super::ListPushFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPushFrontResponse>,
            tonic::Status,
        >;
        async fn list_push_back(
            &self,
            request: tonic::Request<super::ListPushBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPushBackResponse>,
            tonic::Status,
        >;
        async fn list_pop_front(
            &self,
            request: tonic::Request<super::ListPopFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPopFrontResponse>,
            tonic::Status,
        >;
        async fn list_pop_back(
            &self,
            request: tonic::Request<super::ListPopBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPopBackResponse>,
            tonic::Status,
        >;
        async fn list_erase(
            &self,
            request: tonic::Request<super::ListEraseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEraseResponse>,
            tonic::Status,
        >;
        async fn list_remove(
            &self,
            request: tonic::Request<super::ListRemoveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRemoveResponse>,
            tonic::Status,
        >;
        async fn list_fetch(
            &self,
            request: tonic::Request<super::ListFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFetchResponse>,
            tonic::Status,
        >;
        async fn list_length(
            &self,
            request: tonic::Request<super::ListLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLengthResponse>,
            tonic::Status,
        >;
        async fn list_concatenate_front(
            &self,
            request: tonic::Request<super::ListConcatenateFrontRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConcatenateFrontResponse>,
            tonic::Status,
        >;
        async fn list_concatenate_back(
            &self,
            request: tonic::Request<super::ListConcatenateBackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConcatenateBackResponse>,
            tonic::Status,
        >;
        async fn list_retain(
            &self,
            request: tonic::Request<super::ListRetainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRetainResponse>,
            tonic::Status,
        >;
        /// Add or Updates new element with its score to the Sorted Set.
        /// If sorted set doesn't exist, a new one is created with the specified
        /// element and its associated score.
        /// If an element exists, then its associate score gets overridden with the one
        /// provided in this operation.
        async fn sorted_set_put(
            &self,
            request: tonic::Request<super::SortedSetPutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetPutResponse>,
            tonic::Status,
        >;
        /// Fetches a subset of elements in the sorted set.
        async fn sorted_set_fetch(
            &self,
            request: tonic::Request<super::SortedSetFetchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetFetchResponse>,
            tonic::Status,
        >;
        /// Gets the specified element and its associated score if it exists in the
        /// sorted set.
        async fn sorted_set_get_score(
            &self,
            request: tonic::Request<super::SortedSetGetScoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetGetScoreResponse>,
            tonic::Status,
        >;
        /// Removes specified elements and their associated scores
        async fn sorted_set_remove(
            &self,
            request: tonic::Request<super::SortedSetRemoveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetRemoveResponse>,
            tonic::Status,
        >;
        /// Changes the score associated with the element by specified amount.
        /// If the provided amount is negative, then the score associated with the
        /// element is decremented.
        /// If the element that needs to be incremented isn't present in the sorted
        /// set, it is added with specified number as the score.
        /// If the set itself doesn't exist then a new one with specified element and
        /// score is created.
        async fn sorted_set_increment(
            &self,
            request: tonic::Request<super::SortedSetIncrementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetIncrementResponse>,
            tonic::Status,
        >;
        /// Gives the rank of an element.
        async fn sorted_set_get_rank(
            &self,
            request: tonic::Request<super::SortedSetGetRankRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetGetRankResponse>,
            tonic::Status,
        >;
        /// Returns length of the sorted set
        async fn sorted_set_length(
            &self,
            request: tonic::Request<super::SortedSetLengthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetLengthResponse>,
            tonic::Status,
        >;
        /// Returns number of elements in the sorted set between a given min and max score
        async fn sorted_set_length_by_score(
            &self,
            request: tonic::Request<super::SortedSetLengthByScoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SortedSetLengthByScoreResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ScsServer<T: Scs> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Scs> ScsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ScsServer<T>
    where
        T: Scs,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cache_client.Scs/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::GetRequest>
                    for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/Set" => {
                    #[allow(non_camel_case_types)]
                    struct SetSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetRequest>
                    for SetSvc<T> {
                        type Response = super::SetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetIfNotExists" => {
                    #[allow(non_camel_case_types)]
                    struct SetIfNotExistsSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SetIfNotExistsRequest>
                    for SetIfNotExistsSvc<T> {
                        type Response = super::SetIfNotExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetIfNotExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_if_not_exists(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetIfNotExistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::DeleteRequest>
                    for DeleteSvc<T> {
                        type Response = super::DeleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/KeysExist" => {
                    #[allow(non_camel_case_types)]
                    struct KeysExistSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::KeysExistRequest>
                    for KeysExistSvc<T> {
                        type Response = super::KeysExistResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeysExistRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).keys_exist(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeysExistSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/Increment" => {
                    #[allow(non_camel_case_types)]
                    struct IncrementSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::IncrementRequest>
                    for IncrementSvc<T> {
                        type Response = super::IncrementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IncrementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).increment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IncrementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/UpdateTtl" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTtlSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::UpdateTtlRequest>
                    for UpdateTtlSvc<T> {
                        type Response = super::UpdateTtlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTtlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_ttl(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTtlSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ItemGetTtl" => {
                    #[allow(non_camel_case_types)]
                    struct ItemGetTtlSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ItemGetTtlRequest>
                    for ItemGetTtlSvc<T> {
                        type Response = super::ItemGetTtlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ItemGetTtlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).item_get_ttl(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ItemGetTtlSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ItemGetType" => {
                    #[allow(non_camel_case_types)]
                    struct ItemGetTypeSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ItemGetTypeRequest>
                    for ItemGetTypeSvc<T> {
                        type Response = super::ItemGetTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ItemGetTypeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).item_get_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ItemGetTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionaryGet" => {
                    #[allow(non_camel_case_types)]
                    struct DictionaryGetSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::DictionaryGetRequest>
                    for DictionaryGetSvc<T> {
                        type Response = super::DictionaryGetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionaryGetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_get(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionaryGetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionaryFetch" => {
                    #[allow(non_camel_case_types)]
                    struct DictionaryFetchSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::DictionaryFetchRequest>
                    for DictionaryFetchSvc<T> {
                        type Response = super::DictionaryFetchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionaryFetchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_fetch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionaryFetchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionarySet" => {
                    #[allow(non_camel_case_types)]
                    struct DictionarySetSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::DictionarySetRequest>
                    for DictionarySetSvc<T> {
                        type Response = super::DictionarySetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionarySetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_set(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionarySetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionaryIncrement" => {
                    #[allow(non_camel_case_types)]
                    struct DictionaryIncrementSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::DictionaryIncrementRequest>
                    for DictionaryIncrementSvc<T> {
                        type Response = super::DictionaryIncrementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionaryIncrementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_increment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionaryIncrementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionaryDelete" => {
                    #[allow(non_camel_case_types)]
                    struct DictionaryDeleteSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::DictionaryDeleteRequest>
                    for DictionaryDeleteSvc<T> {
                        type Response = super::DictionaryDeleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionaryDeleteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionaryDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/DictionaryLength" => {
                    #[allow(non_camel_case_types)]
                    struct DictionaryLengthSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::DictionaryLengthRequest>
                    for DictionaryLengthSvc<T> {
                        type Response = super::DictionaryLengthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DictionaryLengthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).dictionary_length(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DictionaryLengthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetFetch" => {
                    #[allow(non_camel_case_types)]
                    struct SetFetchSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetFetchRequest>
                    for SetFetchSvc<T> {
                        type Response = super::SetFetchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetFetchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set_fetch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetFetchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetUnion" => {
                    #[allow(non_camel_case_types)]
                    struct SetUnionSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetUnionRequest>
                    for SetUnionSvc<T> {
                        type Response = super::SetUnionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetUnionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set_union(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetUnionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetDifference" => {
                    #[allow(non_camel_case_types)]
                    struct SetDifferenceSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetDifferenceRequest>
                    for SetDifferenceSvc<T> {
                        type Response = super::SetDifferenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDifferenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_difference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDifferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetContains" => {
                    #[allow(non_camel_case_types)]
                    struct SetContainsSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetContainsRequest>
                    for SetContainsSvc<T> {
                        type Response = super::SetContainsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetContainsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_contains(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetContainsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SetLength" => {
                    #[allow(non_camel_case_types)]
                    struct SetLengthSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SetLengthRequest>
                    for SetLengthSvc<T> {
                        type Response = super::SetLengthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetLengthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).set_length(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetLengthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListPushFront" => {
                    #[allow(non_camel_case_types)]
                    struct ListPushFrontSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListPushFrontRequest>
                    for ListPushFrontSvc<T> {
                        type Response = super::ListPushFrontResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPushFrontRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_push_front(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPushFrontSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListPushBack" => {
                    #[allow(non_camel_case_types)]
                    struct ListPushBackSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListPushBackRequest>
                    for ListPushBackSvc<T> {
                        type Response = super::ListPushBackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPushBackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_push_back(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPushBackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListPopFront" => {
                    #[allow(non_camel_case_types)]
                    struct ListPopFrontSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListPopFrontRequest>
                    for ListPopFrontSvc<T> {
                        type Response = super::ListPopFrontResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPopFrontRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_pop_front(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPopFrontSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListPopBack" => {
                    #[allow(non_camel_case_types)]
                    struct ListPopBackSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListPopBackRequest>
                    for ListPopBackSvc<T> {
                        type Response = super::ListPopBackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPopBackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_pop_back(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPopBackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListErase" => {
                    #[allow(non_camel_case_types)]
                    struct ListEraseSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListEraseRequest>
                    for ListEraseSvc<T> {
                        type Response = super::ListEraseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEraseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_erase(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListEraseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListRemove" => {
                    #[allow(non_camel_case_types)]
                    struct ListRemoveSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListRemoveRequest>
                    for ListRemoveSvc<T> {
                        type Response = super::ListRemoveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRemoveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListFetch" => {
                    #[allow(non_camel_case_types)]
                    struct ListFetchSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListFetchRequest>
                    for ListFetchSvc<T> {
                        type Response = super::ListFetchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFetchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_fetch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFetchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListLength" => {
                    #[allow(non_camel_case_types)]
                    struct ListLengthSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListLengthRequest>
                    for ListLengthSvc<T> {
                        type Response = super::ListLengthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLengthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_length(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListLengthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListConcatenateFront" => {
                    #[allow(non_camel_case_types)]
                    struct ListConcatenateFrontSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::ListConcatenateFrontRequest>
                    for ListConcatenateFrontSvc<T> {
                        type Response = super::ListConcatenateFrontResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConcatenateFrontRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_concatenate_front(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConcatenateFrontSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListConcatenateBack" => {
                    #[allow(non_camel_case_types)]
                    struct ListConcatenateBackSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::ListConcatenateBackRequest>
                    for ListConcatenateBackSvc<T> {
                        type Response = super::ListConcatenateBackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConcatenateBackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_concatenate_back(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConcatenateBackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/ListRetain" => {
                    #[allow(non_camel_case_types)]
                    struct ListRetainSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::ListRetainRequest>
                    for ListRetainSvc<T> {
                        type Response = super::ListRetainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRetainRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_retain(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRetainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetPut" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetPutSvc<T: Scs>(pub Arc<T>);
                    impl<T: Scs> tonic::server::UnaryService<super::SortedSetPutRequest>
                    for SortedSetPutSvc<T> {
                        type Response = super::SortedSetPutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetPutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_put(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetPutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetFetch" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetFetchSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetFetchRequest>
                    for SortedSetFetchSvc<T> {
                        type Response = super::SortedSetFetchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetFetchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_fetch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetFetchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetGetScore" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetGetScoreSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetGetScoreRequest>
                    for SortedSetGetScoreSvc<T> {
                        type Response = super::SortedSetGetScoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetGetScoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_get_score(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetGetScoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetRemove" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetRemoveSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetRemoveRequest>
                    for SortedSetRemoveSvc<T> {
                        type Response = super::SortedSetRemoveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetRemoveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_remove(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetIncrement" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetIncrementSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetIncrementRequest>
                    for SortedSetIncrementSvc<T> {
                        type Response = super::SortedSetIncrementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetIncrementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_increment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetIncrementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetGetRank" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetGetRankSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetGetRankRequest>
                    for SortedSetGetRankSvc<T> {
                        type Response = super::SortedSetGetRankResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetGetRankRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_get_rank(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetGetRankSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetLength" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetLengthSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetLengthRequest>
                    for SortedSetLengthSvc<T> {
                        type Response = super::SortedSetLengthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetLengthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_length(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetLengthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cache_client.Scs/SortedSetLengthByScore" => {
                    #[allow(non_camel_case_types)]
                    struct SortedSetLengthByScoreSvc<T: Scs>(pub Arc<T>);
                    impl<
                        T: Scs,
                    > tonic::server::UnaryService<super::SortedSetLengthByScoreRequest>
                    for SortedSetLengthByScoreSvc<T> {
                        type Response = super::SortedSetLengthByScoreResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SortedSetLengthByScoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sorted_set_length_by_score(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SortedSetLengthByScoreSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Scs> Clone for ScsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Scs> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Scs> tonic::server::NamedService for ScsServer<T> {
        const NAME: &'static str = "cache_client.Scs";
    }
}
