use crate::store::Store;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, Error> {
        let id = Uuid::new_v4();

        // TODO add encryption in password

        let user = User {
            username,
            password,
            id: id.to_string(),
        };

        let _ = diesel::insert_into(crate::schema::user::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut self.conn);

        Ok(user.id)
    }

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<bool, Error> {
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password { Ok(false) }
        else { Ok(true) }
    }
}
