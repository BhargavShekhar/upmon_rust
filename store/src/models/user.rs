use crate::store::Store;
use diesel::{prelude::*, result::Error};
use rand::rngs::OsRng;
use uuid::Uuid;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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
        let id = Uuid::new_v4().to_string();

        let salt = SaltString::generate(&mut OsRng);

        let argon2: Argon2<'_> = Argon2::default();

        let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                eprint!("Password hashing failed: {:?}", e);
                Error::RollbackTransaction
            })?
            .to_string();

        let user = User {
            username,
            password: hashed_password,
            id
        };

        diesel::insert_into(crate::schema::user::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(user.id)
    }

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<String, Error> {
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        let parsed_hash= match PasswordHash::new(&user_result.password) {
            Ok(hash) => hash,
            Err(err) => {
                eprint!("Error parsing hash {:?}", err);
                return Err(Error::RollbackTransaction);
            }
        };

        let argon2 = Argon2::default();

        if argon2.verify_password(input_password.as_bytes(), &parsed_hash).is_ok() {
            Ok(user_result.id)
        }
        
        else {
            Err(Error::NotFound)
        }
    }
}
