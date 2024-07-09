pub use indexmap::IndexMap;

pub use crate::record::*;

#[derive(Debug)]
pub enum TableError {
    MismatchedTypes,
    InvalidField,
    InvalidOperator,
    IncorrectFieldNames,
    IncorrectFieldTypes,
    TableHasNoFields,
}

type Result<T> = std::result::Result<T, TableError>;

#[derive(PartialEq)]
pub enum Operator {
    GreaterThan,
    EqualTo,
    LessThan,
}

pub fn create_comp_func<T: Ord>(operator: Operator) -> fn(&T, &T) -> bool {
    match operator {
        Operator::GreaterThan => |a: &T, b: &T| a > b,
        Operator::EqualTo => |a: &T, b: &T| a == b,
        Operator::LessThan => |a: &T, b: &T| a < b,
    }
}
/// Creates an [`IndexMap`] containing field names and 
/// corresponding [`ValueType`]s. Arguments are treated
/// as pairs of field names and types. For example:
/// ```
/// field_types!("ID", "uint", "Name", "str", "Age", "uint");
/// ```
#[macro_export]
macro_rules! field_types{
    ( $( $field_name:expr, $field_type_str:expr ),* ) => {
        {
            let mut field_types = IndexMap::new();
            $(
                let field_type = match $field_type_str {
                    "str" => ValueType::Str,
                    "int" => ValueType::Int,
                    "uint" => ValueType::UInt,
                    "float" => ValueType::Float,
                    _ => panic!("Invalid field type"),
                };
                field_types.insert($field_name, field_type);
            )*
            field_types
        }
    };
}

pub struct Table<'a> {
    pub field_types: Option<IndexMap<&'a str, ValueType>>,
    records: Vec<Record<'a>>,
}

impl<'a> Table<'a> {
    pub fn new(field_types: IndexMap<&'a str, ValueType>) -> Self {
        Self {
            field_types: Some(field_types),
            records: Vec::new(),
        }
    }

    /// Creates a new [`Table`] with no set field types.
    /// Field types will instead be set according to the
    /// first record is added.
    pub fn new_generic() -> Self {
        Self {
            field_types: None,
            records: Vec::new(),
        }
    }

    pub fn record_cnt(&self) -> usize {
        self.records.len()
    }

    /// Adds a [`Record`] after checking if it matches with the field types of the table.
    pub fn add_record(&mut self, record: Record<'a>) -> Result<()> {
        let Some(field_types) = &self.field_types else {
            let mut field_types = IndexMap::new();
            for (field_name, value) in record.clone().into_iter() {
                field_types.insert(field_name, value.to_type());
            }
            self.field_types = Some(field_types);

            self.records.push(record);
            return Ok(());
        };

        for (field_name, expected_field_type) in field_types.iter() {
            let Some(value) = record.get_field_as_value(field_name) else {
                return Err(TableError::IncorrectFieldNames);
            };
            if value.to_type() != *expected_field_type {
                return Err(TableError::IncorrectFieldTypes);
            }
        }

        self.records.push(record);
        Ok(())
    }

    /// Uses [`Table::add_record()`] to add each record.
    pub fn add_records(&mut self, records: Vec<Record<'a>>) -> Result<()> {
        for record in records {
            self.add_record(record)?;
        }
        Ok(())
    }

    pub fn remove_record(&mut self, idx: usize) -> Option<Record<'a>> {
        if idx >= self.record_cnt() {
            return None;
        }
        Some(self.records.remove(idx))
    }

    pub fn get_record(&'a self, idx: usize) -> Option<&'a Record<'a>> {
        if idx >= self.record_cnt() {
            return None;
        }
        Some(&self.records[idx])
    }

    pub fn get_field<T>(&'a self, record_idx: usize, field_name: &'a str) -> Result<T>
    where
        T: TryFrom<&'a Value<'a>, Error = TableError>,
    {
        let Some(value) = self.records[record_idx].get_field_as_value(field_name) else {
            return Err(TableError::InvalidField);
        };

        value.try_into()
    }

    pub fn change_field<T>(
        &'a mut self,
        record_idx: usize,
        field_name: &'a str,
        value: T,
    ) -> Result<()>
    where
        T: Into<Value<'a>>,
        T: Copy,
    {
        let Some(record) = self.records[record_idx].get_mut_field_as_value(field_name) else {
            return Err(TableError::InvalidField);
        };

        if record.to_type() != (&value.into()).to_type() {
            return Err(TableError::MismatchedTypes);
        }

        *record = value.into();
        Ok(())
    }

    /// Returns a new table containing the specified fields of each record in this table.
    pub fn get_fields(&'a self, field_names: Vec<&'a str>) -> Result<Table<'a>> {
        let Some(_) = &self.field_types else {
            if field_names.len() > 0 {
                return Err(TableError::TableHasNoFields);
            }
            return Ok(Table::new_generic())
        };

        let mut new_table = Table::new_generic();
        for record in self.iter() {
            let mut new_record = Record::new();
            for field_name in field_names.iter() {
                let Some(value) = record.get_field_as_value(field_name) else {
                    return Err(TableError::InvalidField);
                };

                new_record.add_field_as_value(field_name, *value);
            }
            new_table.add_record(new_record).unwrap();
        }

        Ok(new_table)
    }

    /// Returns a new table containing all records which fit the condition specified.
    pub fn get_records_where<T>(
        &'a self,
        field_name: &'a str,
        operator: Operator,
        comp_value: T,
    ) -> Result<Table<'a>>
    where
        T: Into<Value<'a>>,
        T: TryFrom<&'a Value<'a>, Error = TableError>,
        T: Ord,
        T: Copy,
    {
        let Some(field_types) = &self.field_types else {
            return Ok(Table::new_generic());
        };
        let Some(expected_type) = field_types.get(field_name) else {
            return Err(TableError::InvalidField);
        };

        if *expected_type != (&comp_value.into()).to_type() {
            return Err(TableError::MismatchedTypes);
        }

        let comp_func = create_comp_func::<T>(operator);

        let mut return_table = Table::new(field_types.clone());
        for record in self.records.iter() {
            // it's fine to unwrap as we checked if field exists before
            let value: T = record.get_field(field_name).unwrap();
            if comp_func(&value, &comp_value) {
                return_table.add_record(record.clone()).unwrap();
            }
        }

        Ok(return_table)
    }

    /// Equivalent to [`Table::get_records_where()`] expect only if the field is equal to the comparison
    /// value. This functions purpose is for strings as they can't be greater than or equal to each
    /// other.
    pub fn get_records_where_eql<T>(
        &'a self,
        field_name: &'a str,
        comp_value: T,
    ) -> Result<Table<'a>>
    where
        T: Into<Value<'a>>,
        T: TryFrom<&'a Value<'a>, Error = TableError>,
        T: PartialEq,
        T: Copy,
    {
        let Some(field_types) = &self.field_types else {
            return Ok(Table::new_generic());
        };
        let Some(_) = field_types.get(field_name) else {
            return Err(TableError::InvalidField);
        };

        let mut return_table = Table::new(field_types.clone());
        for record in self.records.iter() {
            // it's fine to unwrap as we checked if field exists before
            let value: T = record.get_field(field_name).unwrap();
            if value == comp_value {
                return_table.add_record(record.clone()).unwrap();
            }
        }

        Ok(return_table)
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a Record<'a>> {
        self.records.iter()
    }
}
