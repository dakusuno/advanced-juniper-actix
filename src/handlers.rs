use actix_web::{web,Error,HttpResponse};
use crate::db::PostgresPool;
use crate::graphql::root::{Schema,RootContext,create_schema};
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;

pub async fn graphql(
    pool:web::Data<PostgresPool>,
    schema:web::Data<Schema>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse,Error>{
    let ctx = RootContext{
        dbpool:pool.get_ref().to_owned()
    };
    let res = web::block(
        move || {
            let res = data.execute_sync(&schema,&ctx);
            Ok::<_,serde_json::error::Error>(serde_json::to_string(&res)?)
        }
    )
        .await
        .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res)
    )
}
pub async fn graphql_playground() -> HttpResponse{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql",None))

}
pub fn register(config:&mut web::ServiceConfig){
    config
        .data(create_schema())
        .route("/graphql",web::post().to(graphql))
        .route("/graphql",web::get().to(graphql_playground));
}