pub use crate::table::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DBError {
    InvalidTable,
    TableError(TableError),
    ForeignTableIsEmpty,
    InvalidForeignKey,
}

impl From<TableError> for DBError {
    fn from(table_error: TableError) -> Self {
        Self::TableError(table_error)
    }
}

type Result<T> = std::result::Result<T, DBError>;

pub struct DB<'a> {
    tables: HashMap<&'a str, Table<'a>>,
}

impl<'a> DB<'a> {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(
        &mut self,
        table_name: &'a str,
        field_types: IndexMap<&'a str, ValueType>,
    ) {
        self.tables.insert(table_name, Table::new(field_types));
    }

    pub fn get_table(&'a self, table_name: &'a str) -> Option<&'a Table<'a>> {
        self.tables.get(table_name)
    }

    /// See [`Table::add_record()`].
    pub fn add_record(&mut self, table_name: &'a str, record: Record<'a>) -> Result<()> {
        let Some(table) = self.tables.get_mut(table_name) else {
            return Err(DBError::InvalidTable);
        };

        table.add_record(record)?;
        Ok(())
    }
    
    /// See [`Table::add_records()`].
    pub fn add_records(&mut self, table_name: &'a str, records: Vec<Record<'a>>) -> Result<()> {
        let Some(table) = self.tables.get_mut(table_name) else {
            return Err(DBError::InvalidTable);
        };

        table.add_records(records)?;
        Ok(())
    }

    /// Joins the tables by replacing the foreign key in the primary table with the fields from the foreign table.
    pub fn join_tables(
        &'a self,
        primary_table_name: &'a str,
        foreign_table_name: &'a str,
        foreign_key: &'a str,
    ) -> Result<Table<'a>> {
        let Some(primary_table) = self.tables.get(primary_table_name) else {
            return Err(DBError::InvalidTable);
        };
        let Some(foreign_table) = self.tables.get(foreign_table_name) else {
            return Err(DBError::InvalidTable);
        };

        let Some(foreign_field_types) = &foreign_table.field_types else {
            return Err(DBError::ForeignTableIsEmpty);
        };
        if !foreign_field_types.contains_key(foreign_key) {
            return Err(DBError::InvalidForeignKey);
        }

        let mut joined_table = Table::new_generic();
        for record in primary_table.iter() {
            let mut joined_record = Record::new();
            for (&field_name, &value) in record.iter() {
                joined_record.add_field(field_name, value);

                if field_name != foreign_key {
                    continue;
                }

                for foreign_record in foreign_table.iter() {
                    if foreign_record.get_field_as_value(foreign_key).unwrap() == &value {
                        joined_record.extend_fields(foreign_record);
                    }
                }
            }
            joined_table.add_record(joined_record).unwrap();
        }

        Ok(joined_table)
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (&&'a str, &'a Table<'a>)> {
        self.tables.iter()
    }
}
