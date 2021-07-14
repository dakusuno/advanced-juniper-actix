
use juniper::{graphql_object,GraphQLInputObject};
use uuid::Uuid;
use crate::graphql::root::RootContext;
use crate::schema::users;
use super::product::Product;
use diesel::prelude::*;
#[derive(Queryable)]
pub struct User{
    pub id : Uuid,
    pub name : String,
    pub email : String,

}

#[graphql_object(Context = RootContext)]
impl User{
    #[graphql(name = "id")]
    fn id(&self)->Uuid{
        self.id
    }
    #[graphql(name="name")]
    fn name(&self) -> &String {
        &self.name
    }
    #[graphql(name="email")]
    fn email(&self) -> &String{
        &self.email
    }
    #[graphql(name="products")]
    fn products(&self,context:&RootContext) -> Vec<Product>{
        use crate::schema::products::dsl::*;
        let mut conn = context.dbpool.get().unwrap();
        let products_user:Vec<Product> = products.filter(user_id.eq(&self.id)).load::<Product>(&conn).expect("fail");
        products_user
    }

}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a>{
    pub name: &'a str,
    pub email : &'a str
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput{
    pub name:String,
    pub email : String
}

