use crate::carer::carer_model::Carer;
use crate::carer::carer_service::create_carer;
use crate::db::establish_connection;
use async_graphql::*;

use super::{
    carer_model::{CreateCarerInput, DeleteCarerInput, FindCarerInput},
    carer_service::{delete_carer, find_all_carers, find_carer},
};

#[derive(Default)]
pub struct CarerQuery;

#[Object]
impl CarerQuery {
    async fn carers(&self) -> Result<Vec<Carer>> {
        let connection = &mut establish_connection();
        Ok(find_all_carers(connection))
    }

    async fn carer(&self, input: FindCarerInput) -> Result<Carer> {
        let connection = &mut establish_connection();
        Ok(find_carer(connection, input))
    }
}

#[derive(Default)]
pub struct CarerMutation;

#[Object]
impl CarerMutation {
    async fn create_carer(&self, input: CreateCarerInput) -> Result<Carer> {
        let connection = &mut establish_connection();
        Ok(create_carer(connection, input))
    }

    async fn delete_carer(&self, input: DeleteCarerInput) -> Result<usize> {
        let connection = &mut establish_connection();
        Ok(delete_carer(connection, input)?)
    }
}
