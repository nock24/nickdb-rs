use std::fmt;

use crate::db::*;

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Str(str) => write!(f, "{str:10}"),
            Self::Int(int) => write!(f, "{int:10}"),
            Self::UInt(uint) => write!(f, "{uint:10}"),
            Self::Float(float) => write!(f, "{float:10}"),
        }
    }
}

impl fmt::Display for Table<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let Some(field_types) = &self.field_types else {
            return Ok(());
        };

        let field_cnt = field_types.len();

        for (i, (&field_name, _)) in field_types.iter().enumerate() {
            if i == field_cnt - 1 {
                write!(f, "{field_name:10}")?;
            } else {
                write!(f, "{field_name:10}|")?;
            }
        }

        for record in self.iter() {
            writeln!(f, "")?;
            for _ in 0..10 * field_cnt + (field_cnt / 2) + 1 {
                write!(f, "-")?;
            }
            writeln!(f, "")?;

            for (i, (&field_name, _)) in field_types.iter().enumerate() {
                let value = record.get_field_as_value(field_name).unwrap();
                if i == field_cnt - 1 {
                    write!(f, "{value}")?;
                } else {
                    write!(f, "{value}|")?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for DB<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (i, (table_name, table)) in self.iter().enumerate() {
            writeln!(f, "{table_name}:")?;
            write!(f, "{table}")?;

            if i != table.record_cnt() - 1 {
                write!(f, "\n\n")?;
            }
        }
        Ok(())
    }
}
