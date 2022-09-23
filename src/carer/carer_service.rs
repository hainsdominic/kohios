use diesel::prelude::*;
use diesel::PgConnection;

use crate::carer::carer_model::NewCarer;

use super::carer_model::Carer;
use super::carer_model::CreateCarerInput;
use super::carer_model::DeleteCarerInput;
use super::carer_model::FindCarerInput;

pub fn find_all_carers(connection: &mut PgConnection) -> Vec<Carer> {
    use crate::schema::carers::dsl::*;
    carers
        .load::<Carer>(connection)
        .expect("Error loading carers")
}

pub fn find_carer(connection: &mut PgConnection, input: FindCarerInput) -> Carer {
    use crate::schema::carers::dsl::*;
    carers
        .find(input.id)
        .first::<Carer>(connection)
        .expect("Error loading carer")
}

pub fn create_carer(conn: &mut PgConnection, input: CreateCarerInput) -> Carer {
    use crate::schema::carers;

    let new_carer = NewCarer { name: &input.name };

    diesel::insert_into(carers::table)
        .values(&new_carer)
        .get_result(conn)
        .expect("Error saving new Carer")
}

pub fn delete_carer(
    conn: &mut PgConnection,
    input: DeleteCarerInput,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::carers::dsl::*;

    diesel::delete(carers.filter(id.eq(input.id))).execute(conn)
}
