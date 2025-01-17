// TODO: use these aliases in other parts of the protocol module
// From experimentation, it seems the protocol's integers are i32s.
#[allow(dead_code)]
pub type JsInt = i32;
// For when we specifically want to guard against negative numbers.
pub type JsUInt = u32;

// For when the docs call for 'number'
pub type JsFloat = f64;

/// Unique script identifier
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ScriptId
pub type ScriptId = String;

/// Experimental
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-StackTraceId
pub type UniqueDebuggerId = String;
