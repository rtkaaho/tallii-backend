use std::ops::Deref;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{Service, TalliiResponse};

use crate::crypto::Crypto;
use crate::errors::Test;
use crate::models::invite_code::{CreateInviteCode, InviteCode};
use crate::models::user::NewUser;
use crate::repositories::invite_code::InviteCodeRepository;
use crate::repositories::user::UserRepository;

/// Gets all invite codes
// pub async fn get_all_invite_codes(pool: web::Data<PgPool>) -> TalliiResponse {
//     // get the invite code repository
//     let repository = InviteCodeRepository::new(pool.deref().clone());

//     // execute the query
//     let all_invite_codes = repository.get_all().await?;

//     // respond with all of the invite codes
//     Ok(HttpResponse::Ok().json(all_invite_codes))
// }

// /// Checks the validity of the invite code
// pub async fn check_invite_code(
//     pool: web::Data<PgPool>,
//     web::Json(code): web::Json<InviteCode>,
// ) -> TalliiResponse {
//     // get the invite code repository
//     let repository = InviteCodeRepository::new(pool.deref().clone());

//     // execute the query
//     let is_valid  = repository.is_valid(&code.id).await?;

//     // if not valid return an error
//     if !is_valid {
//         Err(TalliiError::InvalidInviteCode)
//     } else {
//         Ok(HttpResponse::Ok().finish())
//     }
// }

/// Checks the validity of the invite code
// pub async fn create_invite_codes(
//     pool: web::Data<PgPool>,
//     web::Json(new_codes): web::Json<CreateInviteCode>,
// ) -> TalliiResponse {
//     // get the invite code repository
//     let repository = InviteCodeRepository::new(pool.deref().clone());

//     // execute the query
//     repository.create_many(new_codes.amount).await?;

//     // response with created
//     Ok(HttpResponse::Created().finish())
// }

/// Logs the user in if the provided credentials are correct
pub async fn login() -> TalliiResponse {
    Err(Test)
}

/// Signs a user up with the provided credentials
// pub async fn signup(
//     pool: web::Data<PgPool>,
//     crypto: web::Data<Crypto>,
//     web::Json(new_user): web::Json<NewUser>,
// ) -> TalliiResponse {

//     // get the user and invite_code repository
//     let user_repo = UserRepository::new(pool.deref().clone());
//     let invite_code_repo = InviteCodeRepository::new(pool.deref().clone());

//     // check to make sure the invite code is valid
//     let is_valid_invite_code = invite_code_repo.is_valid(&new_user.invite_code).await?;

//     println!("{:?}", is_valid_invite_code);

//     // if not valid return an InvalidInviteCode error
//     // if !is_valid_invite_code {
//         return Err(TalliiError::InternalServerError);
//     // }

//     // check to make sure the invite code is not taken
//     // if let None = user_repo.get_by_invite_code(&new_user.invite_code).await? {
//     //     return Err(TalliiError::InvalidInviteCode); 
//     // }

//     // create the new user in the database
//     // let created_user = user_repo.create(new_user, &crypto).await?;

//     // create a new jwt token for that user
//     // let token = crypto.generate_jwt(created_user.user_id, created_user.username);

//     // respond with the newly created token
//     Ok(HttpResponse::Ok().finish())
// }

pub struct Auth;

impl Service for Auth {
    fn define_routes(cfg: &mut web::ServiceConfig) {
        cfg
        // cfg.service(
        //     web::resource("/invite-codes")
        //         .route(web::post().to(check_invite_code))
        //         .route(web::get().to(get_all_invite_codes)),
        // )
        // .service(web::resource("/invite-codes/new").route(web::post().to(create_invite_codes)))
        .service(web::resource("/login").route(web::post().to(login)));
        // .service(web::resource("/signup").route(web::post().to(signup)));
    }
}
