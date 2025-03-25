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
        Box::new(CreateTable { schema: schema })
    }
}

impl<T: Transaction> Executor<T> for CreateTable {
    fn executor(&self, txn: &mut T) -> Result<ResultSet> {
        todo!()
    }
}
