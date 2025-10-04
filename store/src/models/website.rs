use chrono::Utc;
use diesel::{prelude::*};
use uuid::Uuid;
use crate::store::Store;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: Option<String>,
    pub time_added: chrono::NaiveDateTime
}

impl Store {
    pub fn create_website(&mut self, url: String, user_id: String) -> Result<Website, diesel::result::Error> {
        let id = Uuid::new_v4();

        let website = Website {
            id: id.to_string(),
            url,
            user_id: Some(user_id),
            time_added: Utc::now().naive_local()
        };

        let website_result = diesel::insert_into(crate::schema::website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website_result)
    }

    pub fn get_website(&mut self, input_id: String) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(input_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;
        
        Ok(website_result)
    }
}