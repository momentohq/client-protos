/// A hint so you can decide a little more in the abstract "can this be retried?""
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RetrySemantic {
    /// Never retry this message without telling the user. (you should infer this as the default)
    NotRetryable = 0,
    /// You can retry this without surfacing an error to the user.
    Retryable = 1,
}
impl RetrySemantic {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RetrySemantic::NotRetryable => "NotRetryable",
            RetrySemantic::Retryable => "Retryable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotRetryable" => Some(Self::NotRetryable),
            "Retryable" => Some(Self::Retryable),
            _ => None,
        }
    }
}
