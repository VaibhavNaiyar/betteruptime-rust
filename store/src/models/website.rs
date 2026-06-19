use crate::store::Store;
use diesel::{prelude::*};
use uuid :: Uuid;
use chrono::NativeDateTime;

#[derive(Queryable , Insertable , Selectable)]
#[diesel(table_name = create::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]


pub struct Website {
    pub id:String,
    pub url:String,
    pub user_id:String , 
    pub time_added: chrono:: NaiveDateTime
}


impl Store {
    pub fn  create_website(&self , user_id:String , url:String) -> Result<Website , diesel::result::Error> {
        let id = Uuid::new_v4();
        let website = Website {
            user_id , 
            url,
            id:id.to_String(),
            time_added:Utc::now().native_utc()
        };

        let website = diesel::insert_into(crate::schema::website::table)
            .values(&website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website(&mut self, input_id: String) -> Result<Website,diesel::result::Error> {
        use crate::Schema::website::dsl::*;

        let website_result = website.filter(id.eq(input_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(website_result)
    }
}