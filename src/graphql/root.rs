use crate::db::PostgresPool;
use diesel::{RunQueryDsl, PgConnection, QueryDsl};
use crate::graphql::user::{User, CreateUserInput, NewUser};
use juniper::{FieldResult, FieldError, RootNode, Context, EmptySubscription};
use uuid::Uuid;
use crate::graphql::product::{Product, CreateProductInput, NewProduct};
use crate::schema::products::dsl::products;
use crate::schema::users::dsl::users;

pub struct RootContext{
    pub dbpool:PostgresPool
}

impl juniper::Context for RootContext{}

pub struct Query;

#[juniper::graphql_object(Context = RootContext)]
impl Query{
    #[graphql(name="listUsers")]
    fn list_users(context:&RootContext)-> FieldResult<Vec<User>>{
        use crate::schema::users::dsl::*;
        let mut conn:&PgConnection = &context.dbpool.get().unwrap();
        let users_result = users.load::<User>(conn);
        graphql_translate(users_result)
    }
    #[graphql(name="finduser")]
    fn detail_user(context:&RootContext,id_user:String)->FieldResult<Option<User>>{
        use crate::schema::users::dsl::*;
        let uuid_user = Uuid::parse_str(&id_user);
        match uuid_user{
            Ok(uuidUser)=>{
                let mut conn:&PgConnection = &context.dbpool.get().unwrap();
                match users.find(uuidUser).get_result::<User>(conn){
                    Ok(user)=>Ok(Some(user)),
                    Err(e)=>match e{
                        diesel::result::Error::NotFound => FieldResult::Ok(None),
                        _ => FieldResult::Err(FieldError::from(e))
                    }
                }
            }
            Err(e)=> {FieldResult::Err(FieldError::from(e))}
        }

    }
    #[graphql(name="listProduct")]
    fn list_product(context:&RootContext)->FieldResult<Vec<Product>>{
        use crate::schema::products::dsl::*;
        let mut conn:&PgConnection= &context.dbpool.get().unwrap();
        let res_product=products.load::<Product>(conn);
        graphql_translate(res_product)
    }
    #[graphql(name="findProduct")]
    fn find_product(context:&RootContext,id_product:String)->FieldResult<Option<Product>>{
        use crate::schema::products::dsl::*;
        let mut cocnn:&PgConnection = &context.dbpool.get().unwrap();
        let uuid_product = Uuid::parse_str(&id_product).expect("failing parse uuid");
        match products.find(uuid_product).get_result::<Product>(cocnn) {
            Ok(product)=>Ok(Some(product)),
            Err(e)=>match e{
                diesel::result::Error::NotFound=>FieldResult::Ok(None),
                _=>FieldResult::Err(FieldError::from(e))
            }
        }
    }
}
pub struct Mutation ;

#[juniper::graphql_object(Context=RootContext)]
impl Mutation{
    #[graphql(name="createUser")]
    fn create_user(context:&RootContext,user: CreateUserInput)->FieldResult<User>{
        use crate::schema::users::dsl::*;
        let mut conn: &PgConnection = &context.dbpool.get().unwrap();
        let new_user = NewUser{
            name:&user.name,
            email:&user.email
        };
        let res = diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn);
        graphql_translate(res)
    }
    #[graphql(name="createProduct")]
    fn create_products(context:&RootContext,product:CreateProductInput)->FieldResult<Product>{
        let mut conn: &PgConnection = &context.dbpool.get().unwrap();
        let new_product = NewProduct{
            user_id: &uuid::Uuid::parse_str(&product.user_id).expect("Cannot parse uuid"),
            name:&product.name,
            price:&product.price
        };
        let res = diesel::insert_into(products)
            .values(&new_product)
            .get_result(conn);
        graphql_translate(res)
    }
}
fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
pub type Schema = RootNode<'static,Query,Mutation,EmptySubscription<RootContext>>;
pub fn create_schema()->Schema{
    Schema::new(Query,Mutation,EmptySubscription::new())
}