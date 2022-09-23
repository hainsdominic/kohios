use crate::models::{Carer, NewCarer};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn create_carer(conn: &mut PgConnection, name: &str) -> Carer {
    use crate::schema::carers;

    let new_carer = NewCarer { name };

    diesel::insert_into(carers::table)
        .values(&new_carer)
        .get_result(conn)
        .expect("Error saving new Carer")
}
