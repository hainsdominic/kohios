use crate::schema::carers::dsl::*;
use crate::{db::establish_connection, schema::carers};
use async_graphql::*;
use diesel::prelude::*;
pub type KohiosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn carers(&self) -> Vec<Carer> {
        let connection = &mut establish_connection();
        carers.load::<Carer>(connection).unwrap()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_carer(&self, input: String) -> Result<Carer> {
        let connection = &mut establish_connection();
        let new_carer = NewCarer { name: &input };
        diesel::insert_into(carers::table)
            .values(&new_carer)
            .get_result(connection)
            .map_err(|e| e.into())
    }
}
