use crate::{
    error::Result,
    sql::{engine::Transaction, schema::Table},
};

use super::{Executor, ResultSet};

pub struct CreateTable {
    schema: Table,
}

impl CreateTable {
    pub fn new(schema: Table) -> Box<CreateTable> {
        Box::new(CreateTable { schema })
    }
}

impl<T: Transaction> Executor<T> for CreateTable {
    fn executor(self: Box<Self>, txn: &mut T) -> Result<ResultSet> {
        let table_name = self.schema.name.clone();
        txn.create_table(self.schema)?;
        Ok(ResultSet::CreateTable { table_name })
    }
}
