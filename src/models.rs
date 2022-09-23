use crate::carer::carer_resolver::{CarerMutation, CarerQuery};
use async_graphql::*;
pub type KohiosSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(CarerQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(CarerMutation);
