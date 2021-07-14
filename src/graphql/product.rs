

use juniper::{graphql_object,GraphQLInputObject};
use uuid::Uuid;
use super::user::User;
use crate::schema::products;
use super::root::RootContext;
use diesel::QueryDsl;

#[derive(Queryable)]
pub struct Product{
    pub id:Uuid,
    pub user_id:Uuid,
    pub name:String,
    pub price:f64
}
#[graphql_object(Context=RootContext)]
impl Product{
    #[graphql(name="id")]
    fn id(&self)-> &Uuid{
        &self.id
    }
    #[graphql(name="userId")]
    fn user_id(&self)->&Uuid{
        &self.user_id
    }
    #[graphql(name="name")]
    fn name(&self)->&str{
        &self.name
    }
    #[graphql(name="user")]
    fn user(&self, context:&RootContext)->Option<User>{
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let conn = context.dbpool.get().unwrap();
        let user_option = users.filter(id.eq(&self.user_id)).first(&conn).optional().unwrap();
        user_option
    }



}
#[derive(Debug,Insertable)]
#[table_name = "products"]
pub struct NewProduct<'a>{
    pub user_id : &'a Uuid,
    pub name : &'a str,
    pub price : &'a f64
}

#[derive(GraphQLInputObject)]
pub struct CreateProductInput{
    pub user_id : String,
    pub name : String,
    pub price : f64
}