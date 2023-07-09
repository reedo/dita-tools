use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// TODO: Document this!
pub enum Pattern {
    Element,
    Attribute {
        content: AttributeName,
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    Group {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    Interleave {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    Choice {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    Optional {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    ZeroOrMore {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    OneOrMore {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    List {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    Mixed {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        open_patterns: OpenPatterns,
    },
    Ref {
        name: String,
        #[serde(flatten)]
        common_atts: CommonAtts,
    },
    ParentRef {
        name: String,
        #[serde(flatten)]
        common_atts: CommonAtts,
    },
    Empty {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    Text {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    Value {
        #[serde(rename = "type")]
        kind: Option<String>,
        #[serde(flatten)]
        common_atts: CommonAtts,
    },
    Data {
        #[serde(rename = "type")]
        kind: String,
        param: Option<Vec<String>>,
        except: Option<Except>,
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    NotAllowed {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    ExternalRef {
        href: String,
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        other: Other,
    },
    Grammar {
        #[serde(flatten)]
        common_atts: CommonAtts,
        #[serde(flatten)]
        content: GrammarContent,
    },
}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub enum AttributeName {
    Name(String),
    NameClass(Option<Vec<String>>),
}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct GrammarContent {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct IncludeContent {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct StartElement {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct DefineElement {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct CombineAtt {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct OpenPatterns {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct OpenPattern {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct NameClass {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct ExceptNameClass {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct OpenNameClasses {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Document this!
pub struct OpenNameClass {}

#[derive(Serialize, Deserialize, Debug)]
/// Attributes that are common to many RNG elements.
pub struct CommonAtts {
    #[serde(rename = "@ns")]
    pub ns: Option<String>,
    #[serde(rename = "@datatypeLibrary")]
    pub datatype_library: Option<String>,
    #[serde(flatten)]
    pub others: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Not sure how to gather up all leftover attributes into this struct.
pub struct Other {}

#[derive(Serialize, Deserialize, Debug)]
/// TODO: Not sure how to do this one 🤔.
pub struct Except {}
