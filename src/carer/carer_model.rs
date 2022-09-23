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

#[derive(InputObject)]
pub struct FindCarerInput {
    pub id: i32,
}

#[derive(InputObject)]
pub struct CreateCarerInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct DeleteCarerInput {
    pub id: i32,
}
