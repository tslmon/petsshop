use actix_session::CookieSession;
use actix_web::{guard, web, *};
use api::actions::ManagementTrait;
// use api::actions::monitoring::AggregationManagementTrait;
use api::common::card::*;
use api::common::category::*;
use api::common::comment::*;
use api::common::order::*;
use api::common::order_item::*;
use api::common::payment::*;
use api::common::product::*;
use api::common::user::*;
use api::common::Health;
use serde::Deserialize;
use utils::rate_limit::RateLimit;

pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimit) {
    cfg.service(
        web::scope("/petsshop/v1")
            // Health Check
            .service(web::scope("/health").route("", web::get().to(route_responder::<Health>)))
            // Building Management
            .service(
                web::scope("/users")
                    .route("", web::post().to(UserApi::create_item))
                    .route("/{user_id}", web::get().to(UserApi::get_item))
                    .route("/{user_id}", web::delete().to(UserApi::delete_item))
                    .route("", web::get().to(UserApi::get_collection))
                    .route("/{user_id}", web::put().to(UserApi::update_item)),
            )
            .service(
                web::scope("/categories")
                    .route("", web::post().to(CategoryApi::create_item))
                    .route("/{category_id}", web::get().to(CategoryApi::get_item))
                    .route("/{category_id}", web::delete().to(CategoryApi::delete_item))
                    .route("", web::get().to(CategoryApi::get_collection))
                    .route("/{category_id}", web::put().to(CategoryApi::update_item)),
            )
            .service(
                web::scope("/products")
                    .route("", web::post().to(ProductApi::create_item))
                    .route("/{product_id}", web::get().to(ProductApi::get_item))
                    .route("/{product_id}", web::delete().to(ProductApi::delete_item))
                    .route("", web::get().to(ProductApi::get_collection))
                    .route("/{product_id}", web::put().to(ProductApi::update_item)),
            )
            .service(
                web::scope("/payments")
                    .route("", web::post().to(PaymentApi::create_item))
                    .route("/{payment_id}", web::get().to(PaymentApi::get_item))
                    .route("/{payment_id}", web::delete().to(PaymentApi::delete_item))
                    .route("", web::get().to(PaymentApi::get_collection))
                    .route("/{payment_id}", web::put().to(PaymentApi::update_item)),
            )
            .service(
                web::scope("/orders")
                    .route("", web::post().to(OrderApi::create_item))
                    .route("/{order_id}", web::get().to(OrderApi::get_item))
                    .route("/{order_id}", web::delete().to(OrderApi::delete_item))
                    .route("", web::get().to(OrderApi::get_collection))
                    .route("/{order_id}", web::put().to(OrderApi::update_item)),
            )
            .service(
                web::scope("/order_items")
                    .route("", web::post().to(OrderItemApi::create_item))
                    .route("/{order_item_id}", web::get().to(OrderItemApi::get_item))
                    .route(
                        "/{order_item_id}",
                        web::delete().to(OrderItemApi::delete_item),
                    )
                    .route("", web::get().to(OrderItemApi::get_collection))
                    .route("/{order_item_id}", web::put().to(OrderItemApi::update_item)),
            )
            .service(
                web::scope("/comments")
                    .route("", web::post().to(CommentApi::create_item))
                    .route("/{comment_id}", web::get().to(CommentApi::get_item))
                    .route("/{comment_id}", web::delete().to(CommentApi::delete_item))
                    .route("", web::get().to(CommentApi::get_collection))
                    .route("/{comment_id}", web::put().to(CommentApi::update_item)),
            )
            .service(
                web::scope("/cards")
                    .route("", web::post().to(CardApi::create_item))
                    .route("/{card_id}", web::get().to(CardApi::get_item))
                    .route("/{card_id}", web::delete().to(CardApi::delete_item))
                    .route("", web::get().to(CardApi::get_collection))
                    .route("/{card_id}", web::put().to(CardApi::update_item)),
            ),
    );
}

async fn route_responder<'a, Data>(data: Data) -> impl Responder
where
    Data: Send + 'static + Responder,
{
    data
}

async fn route_responder_with_payload<'a, Data>(
    data: web::Either<web::Form<Data>, web::Json<Data>>,
) -> impl Responder
where
    Data: Deserialize<'a> + Send + 'static + Responder,
{
    data.into_inner()
}
