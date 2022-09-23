use crate::carer::carer_model::Carer;
use crate::carer::carer_service::create_carer;
use crate::db::establish_connection;
use crate::schema::carers::dsl::*;
use async_graphql::*;
use diesel::prelude::*;

use super::carer_service::{delete_carer, find_all_carers};

#[derive(Default)]
pub struct CarerQuery;

#[Object]
impl CarerQuery {
    async fn carers(&self) -> Result<Vec<Carer>> {
        let connection = &mut establish_connection();
        Ok(find_all_carers(connection))
    }

    async fn carer(&self, input: i32) -> Result<Carer> {
        let connection = &mut establish_connection();
        carers
            .find(input)
            .first::<Carer>(connection)
            .map_err(|e| e.into())
    }
}

#[derive(Default)]
pub struct CarerMutation;

#[Object]
impl CarerMutation {
    async fn create_carer(&self, input: String) -> Result<Carer> {
        let connection = &mut establish_connection();
        Ok(create_carer(connection, &input))
    }

    async fn delete_carer(&self, input: i32) -> Result<usize> {
        let connection = &mut establish_connection();
        Ok(delete_carer(connection, input)?)
    }
}
