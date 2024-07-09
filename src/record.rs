use std::collections::HashMap;

use crate::table::TableError;
pub use crate::value::*;

/// A hash map containing field names and corresponding [`Value`]s.
#[derive(Clone)]
pub struct Record<'a> {
    fields: HashMap<&'a str, Value<'a>>,
}

impl<'a> Record<'a> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn extend_fields(&mut self, record: &'a Record<'a>) {
        self.fields.extend(record.iter());
    }

    pub fn add_field<T>(&mut self, field_name: &'a str, value: T)
    where
        T: Into<Value<'a>>,
    {
        self.fields.insert(field_name, value.into());
    }

    pub fn add_field_as_value(&mut self, field_name: &'a str, value: Value<'a>) {
        self.fields.insert(field_name, value);
    }

    pub fn get_field_as_value(&'a self, field_name: &'a str) -> Option<&'a Value<'a>> {
        self.fields.get(field_name)
    }

    pub fn get_mut_field_as_value(&'a mut self, field_name: &'a str) -> Option<&'a mut Value<'a>> {
        self.fields.get_mut(field_name)
    }

    /// Equivalent to [`Record::get_field_as_value()`] except the value is then converted to a [`T`].
    pub fn get_field<T>(&'a self, field_name: &'a str) -> Result<T, TableError>
    where
        T: TryFrom<&'a Value<'a>, Error = TableError>,
    {
        let Some(value) = self.fields.get(field_name) else {
            return Err(TableError::InvalidField);
        };

        value.try_into()
    }

    pub fn contains_field(&'a self, field_name: &'a str) -> bool {
        self.fields.contains_key(field_name)
    }

    pub fn field_cnt(&self) -> usize {
        self.fields.len()
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (&&'a str, &'a Value<'a>)> {
        self.fields.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = (&'a str, Value<'a>)> {
        self.fields.into_iter()
    }
}


/// Creates a [`Record`]. Arguments are treated as
/// pairs of field names and values. For example: 
/// ```
/// record!("ID", 5, "Name", "Edward", "Age", 34);
/// ```
#[macro_export]
macro_rules! record{
    ( $( $field_name:expr, $value:expr ),* ) => {
        {
            let mut record = Record::new();
            $(
                record.add_field($field_name, $value);
            )*
            record
        }
    };
}
