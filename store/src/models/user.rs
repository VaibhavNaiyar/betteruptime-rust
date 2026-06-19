user crate::{schema::user , store::Store};
use crate::store::Store;
use diesel::prelude::*;

#[derive(Queryable , Insertable , Selectable)]
#[diesel(table_name = create::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]


struct User {
    id: String,
    username: String,
    password: String
}

impl Store {
    pub fn  sign_up(&mut self , usernamer:String , password: String) -> Result<String, diesel::result::Error> {
        let id = Uuid::new_v4();
        let u = User {
            username,
            password,
            id:id.to_string()
        };
        diesel::insert_into(crate::Schema::user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        
        Ok(id.toString())

    }

    pub fn sign_in(&mut self, input_username:String , input_password:String) -> Result<bool , diesel::result::Error>{
        use crate::schema::user::dsl::*;

        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;


        if user_result.paswword != input_password {
            return Ok(false);
        }
        Ok(true)
    }
}