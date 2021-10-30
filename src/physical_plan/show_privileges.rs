use std::sync::{Mutex, Arc};

use arrow::array::{StringArray};
use arrow::datatypes::{SchemaRef};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

use crate::core::global_context::GlobalContext;
use crate::mysql::error::{MysqlResult};
use crate::core::output::ResultSet;

pub struct ShowPrivileges {
    global_context: Arc<Mutex<GlobalContext>>,
}

impl ShowPrivileges {
    pub fn new(
        global_context: Arc<Mutex<GlobalContext>>,
    ) -> Self {
        Self {
            global_context,
        }
    }

    pub fn execute(&self) -> MysqlResult<ResultSet> {
        let schema = SchemaRef::new(Schema::new(vec![
            Field::new("Privilege", DataType::Utf8, false),
            Field::new("Context", DataType::Utf8, false),
            Field::new("Comment", DataType::Utf8, false),
        ]));

        let column_values0 = StringArray::from(vec![
            "Alter", "Alter routine", "Create",
            "TABLE_ENCRYPTION_ADMIN",
        ]);
        let column_values1 = StringArray::from(vec![
            "Tables", "Functions,Procedures", "Databases,Tables,Indexes",
            "Server Admin",
        ]);
        let column_values2 = StringArray::from(vec![
            "To alter the table", "To alter or drop stored functions/procedures", "To create new databases and tables",
            "",
        ]);
        let record_batch = RecordBatch::try_new(schema.clone(), vec![
            Arc::new(column_values0),
            Arc::new(column_values1),
            Arc::new(column_values2),
        ]).unwrap();

        Ok(ResultSet::new(schema, vec![record_batch]))
    }
}
