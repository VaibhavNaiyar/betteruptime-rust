use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::User)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserRecord {
    id: String,
    username: String,
    password: String
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4();
        let u = UserRecord {
            username,
            password,
            id: id.to_string()
        };
        diesel::insert_into(crate::schema::User::table)
            .values(&u)
            .returning(UserRecord::as_returning())
            .get_result(&mut self.conn)?;

        Ok(id.to_string())
    }

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<String, diesel::result::Error> {
        use crate::schema::User::dsl;

        let user_result = dsl::User
            .filter(dsl::username.eq(input_username))
            .select(UserRecord::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password {
            return Err(diesel::result::Error::NotFound);
        }
        Ok(user_result.id)
    }
}
