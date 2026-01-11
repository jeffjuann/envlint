mod file;

pub use file::{FileLine, LineType};

mod env;

pub use env::{Variable, VariableRangeList, VariableRange, VariableTag, VariableTagErrorKind, VariableType, VariableTypeErrorKind, VariableMetadata};

mod variable_collection;

pub use variable_collection::VariableCollection;