use crate::{error::Result, sql::schema::Table};

use super::{Executor, ResultSet};

pub struct CreateTable {
    schema: Table,
}

impl CreateTable {
    pub fn new(schema: Table) -> Box<CreateTable> {
        Box::new(CreateTable { schema: schema })
    }
}

impl Executor for CreateTable {
    fn executor(&self) -> Result<ResultSet> {
        todo!()
    }
}
