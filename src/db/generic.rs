// Generic functions for Diesel.
// If it ever breaks, either try to debug or replace it with macros.
// In last resort, just write normal code…

use crate::DbConn;
use crate::errors::{ErrorKind, ServerError, throw};

use diesel::associations::HasTable;
use diesel::dsl::Find;
use diesel::prelude::*;
use diesel::query_builder::{InsertStatement, DeleteStatement, IntoUpdateTarget};
use diesel::query_dsl::methods::{FindDsl, ExecuteDsl};
use diesel::query_dsl::LoadQuery;

type DeleteFindStatement<F> =
DeleteStatement<<F as HasTable>::Table, <F as IntoUpdateTarget>::WhereClause>;

// Generic implementation for a SELECT WHERE table.id=X clause.
// The row must exist, else it'll return Err.
// Also returns Err in case of DB error.
pub fn db_get_row<'a, Model, Table>(
    conn: &mut DbConn,
    table: Table,
    req_id: i32,
    ) -> Result<Model, ServerError>
where
    Table: FindDsl<i32>,
    Find<Table, i32>: LoadQuery<'a, DbConn, Model>,
{
    table.find(req_id).get_result::<Model>(conn).map_err(|e| {
        throw(ErrorKind::DbFail, e.to_string())
    })
}

// Generic implementation for a DELETE WHERE table.id=X clause.
// Also returns Err in case of DB error.
pub fn db_remove<Tbl, Pk>(conn: &mut DbConn, table: Tbl, pk: Pk) -> Result<(), ServerError>
where
    Tbl: FindDsl<Pk>,
    Find<Tbl, Pk>: IntoUpdateTarget,
    DeleteFindStatement<Find<Tbl, Pk>>: ExecuteDsl<DbConn>,
{
    let find = table.find(pk);
    let delete = diesel::delete(find);
    delete.execute(conn).map_err(|e| {
        throw(ErrorKind::DbFail, e.to_string())
    })?;
    Ok(())
}

// Returns all rows from a table.
// If the table is empty, it does not return Err, but an empty Vec instead.
pub fn db_get_all<'a, Model, Table>(conn: &mut DbConn, table: Table) -> Result<Vec<Model>, ServerError>
where
    Table: LoadQuery<'a, DbConn, Model>,
{
    table.get_results::<Model>(conn).map_err(|e| {
        throw(ErrorKind::DbFail, e.to_string())
    })
}

// Inserts a record into DB, from an Insertable model.
// Returns the inserted record as a table struct.
// The returned record usually comes with the id field, contrary to the given data.
// This function implies support of the RETURNING clause.
pub fn db_insert<'a, T: Table, V, Model>(conn: &mut DbConn, table: T, aggregate: V) -> Result<Model, ServerError>
where
    V: diesel::Insertable<T>,
    InsertStatement<T, V::Values>: LoadQuery<'a, DbConn, Model>,
{
    diesel::insert_into(table)
        .values(aggregate)
        .get_result(conn).map_err(|e| {
            throw(ErrorKind::DbFail, e.to_string())
        })
}

