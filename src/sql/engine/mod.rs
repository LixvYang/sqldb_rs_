use crate::error::Result;

use super::{exexutor::ResultSet, parser::Parser, plan::Plan, schema::Table, types::Row};

mod kv;

pub trait Engine: Clone {
    type Transaction: Transaction;

    fn begin(&self) -> Result<Self::Transaction>;

    fn session(&self) -> Result<Session<Self>> {
        Ok(Session {
            engine: self.clone(),
        })
    }
}

// 抽象引擎
/**
 * 底层可以接入 KV 存储引擎
 */
pub trait Transaction {
    fn commit(&self) -> Result<()>;

    fn rollback(&self) -> Result<()>;

    fn create_row(&mut self, table: String, row: Row) -> Result<()>;

    fn scan_table(&mut self, table_name: String) -> Result<Vec<Row>>;

    fn create_table(&mut self, table_name: String) -> Result<()>;

    fn get_table(&mut self, table_name: String) -> Result<Option<Table>>;
}

pub struct Session<E: Engine> {
    engine: E,
}

impl<E: Engine> Session<E> {
    pub fn exexutor(&mut self, sql: &str) -> Result<ResultSet> {
        match Parser::new(sql).parse()? {
            stmt => {
                let mut txn = self.engine.begin()?;
                // 构建执行计划
                match Plan::build(stmt).executor(&mut txn) {
                    Ok(result) => {
                        txn.commit()?;
                        Ok(result)
                    }
                    Err(err) => {
                        txn.rollback();
                        Err(err)
                    }
                }
            }
        }
    }
}
