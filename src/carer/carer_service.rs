use diesel::prelude::*;
use diesel::PgConnection;

use crate::carer::carer_model::NewCarer;

use super::carer_model::Carer;

pub fn find_all_carers(connection: &mut PgConnection) -> Vec<Carer> {
    use crate::schema::carers::dsl::*;
    carers
        .load::<Carer>(connection)
        .expect("Error loading carers")
}

pub fn create_carer(conn: &mut PgConnection, name: &str) -> Carer {
    use crate::schema::carers;

    let new_carer = NewCarer { name };

    diesel::insert_into(carers::table)
        .values(&new_carer)
        .get_result(conn)
        .expect("Error saving new Carer")
}

pub fn delete_carer(
    conn: &mut PgConnection,
    deleted_id: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::carers::dsl::*;

    diesel::delete(carers.filter(id.eq(deleted_id))).execute(conn)
}
