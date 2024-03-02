/// Rule defines the action(s) to take when a source is matched
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    #[prost(enumeration = "PolicyAction", tag = "1")]
    pub action: i32,
    #[prost(message, optional, tag = "2")]
    pub selector: ::core::option::Option<Selector>,
    #[prost(message, optional, tag = "3")]
    pub updates: ::core::option::Option<Update>,
}
/// Update contains updates to the matched build step after rule is applied
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub attrs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Selector identifies a source to match a policy to
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Selector {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// MatchType is the type of match to perform on the source identifier
    #[prost(enumeration = "MatchType", tag = "2")]
    pub match_type: i32,
    #[prost(message, repeated, tag = "3")]
    pub constraints: ::prost::alloc::vec::Vec<AttrConstraint>,
}
/// AttrConstraint defines a constraint on a source attribute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttrConstraint {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(enumeration = "AttrMatch", tag = "3")]
    pub condition: i32,
}
/// Policy is the list of rules the policy engine will perform
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Currently 1
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
}
/// PolicyAction defines the action to take when a source is matched
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolicyAction {
    Allow = 0,
    Deny = 1,
    Convert = 2,
}
impl PolicyAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolicyAction::Allow => "ALLOW",
            PolicyAction::Deny => "DENY",
            PolicyAction::Convert => "CONVERT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALLOW" => Some(Self::Allow),
            "DENY" => Some(Self::Deny),
            "CONVERT" => Some(Self::Convert),
            _ => None,
        }
    }
}
/// AttrMatch defines the condition to match a source attribute
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttrMatch {
    Equal = 0,
    Notequal = 1,
    Matches = 2,
}
impl AttrMatch {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttrMatch::Equal => "EQUAL",
            AttrMatch::Notequal => "NOTEQUAL",
            AttrMatch::Matches => "MATCHES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EQUAL" => Some(Self::Equal),
            "NOTEQUAL" => Some(Self::Notequal),
            "MATCHES" => Some(Self::Matches),
            _ => None,
        }
    }
}
/// Match type is used to determine how a rule source is matched
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchType {
    /// WILDCARD is the default matching type.
    /// It may first attempt to due an exact match but will follow up with a wildcard match
    /// For something more powerful, use REGEX
    Wildcard = 0,
    /// EXACT treats the source identifier as a litteral string match
    Exact = 1,
    /// REGEX treats the source identifier as a regular expression
    /// With regex matching you can also use match groups to replace values in the destination identifier
    Regex = 2,
}
impl MatchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchType::Wildcard => "WILDCARD",
            MatchType::Exact => "EXACT",
            MatchType::Regex => "REGEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WILDCARD" => Some(Self::Wildcard),
            "EXACT" => Some(Self::Exact),
            "REGEX" => Some(Self::Regex),
            _ => None,
        }
    }
}
