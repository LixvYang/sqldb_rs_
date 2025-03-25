use crate::sql::{engine::Transaction, parser::ast::Expression};

use super::Executor;

pub struct Insert {
    table_name: String,
    columes: Vec<String>,
    values: Vec<Vec<Expression>>,
}

impl Insert {
    pub fn new(
        table_name: String,
        columes: Vec<String>,
        values: Vec<Vec<Expression>>,
    ) -> Box<Self> {
        Box::new(Self {
            table_name: table_name,
            columes: columes,
            values: values,
        })
    }
}

impl<T: Transaction> Executor<T> for Insert {
    fn executor(&self, txn: &mut T) -> crate::error::Result<super::ResultSet> {
        todo!()
    }
}
