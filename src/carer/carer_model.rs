use crate::schema::carers;
use async_graphql::*;
use diesel::prelude::*;

#[derive(Queryable, SimpleObject)]
pub struct Carer {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = carers)]
pub struct NewCarer<'a> {
    pub name: &'a str,
}

pub struct DeleteCarer {
    pub id: i32,
}
