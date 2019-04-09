use actix_web::*;
use app::AppState;
use handler::{ResponseTypes, Queries, UserData};
use handler::AuthData;
use utils::create_token;
use actix_web::middleware::identity::RequestIdentity;
use handler::LoggedUser;


use futures::{Future, Stream};
use handler::handle_multipart_item;
use handler::{CreateReview};
use handler::{CreateImage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: u8
}

pub fn register_user((user_data, state): (Json<UserData>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    let user_data = user_data.into_inner();
    let msg = Queries::RegisterUser(user_data);

    state.db.send(msg)
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Status(status)) => {
                if status == 1 {
                    return Ok(HttpResponse::Ok().json("Đăng ký thành công!"));
                } else if status == 2 {
                    return Ok(HttpResponse::InternalServerError().json("Lỗi không hoạt động!"))
                } else {
                    return Ok(HttpResponse::BadRequest().json("Email đã được đăng ký!"));
                }
            },
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống đăng ký tài khoản bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Đăng ký tài khoản thất bại!")),
        })
        .responder()
}

pub fn get_images(state: State<AppState>) -> FutureResponse<HttpResponse> {

    state.db.send(Queries::GetAllImages)
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::AllImages(images)) => Ok(HttpResponse::Ok().json(images)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống lấy danh sách hình ảnh bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Lấy danh sách hình ảnh bị lỗi!")),
        })
        .responder()
}
pub fn get_full_images(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::GetAllBackgrounds)
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Backgrounds(reviews)) => Ok(HttpResponse::Ok().json(reviews)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống lấy danh sách hình ảnh bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Lấy danh sách hình ảnh bị lỗi")),
        })
        .responder()
}
pub fn delete_image((id, state): (Path<u32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::DeleteBackground(id.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Status(status)) => {
                if status == 1 {
                    Ok(HttpResponse::Ok().json("Xoá thành công!"))
                } else {
                    Ok(HttpResponse::BadRequest().json("Xoá thất bại!"))
                }
            },
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống xoá hình ảnh bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Xoá hình ảnh bị lỗi")),
        })
        .responder()
}


pub fn change_image_status((id, _status, state): (Path<u32>, Json<Status>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::ChangeBackgroundStatus(id.into_inner(), _status.into_inner().status))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Status(status)) => {
                if status == 1 {
                    Ok(HttpResponse::Ok().json("Ok!"))
                } else {
                    Ok(HttpResponse::BadRequest().json("Thất bại!"))
                }
            },
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống thay đổi trạng thái hình ảnh bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Thay đổi trạng thái hình ảnh bị lỗi")),
        })
        .responder()
}


pub fn create_full_image((image, state): (Json<CreateImage>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::CreateBackground(image.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Background(background)) => Ok(HttpResponse::Ok().json(background)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Tạo mới bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Tạo mới bị lỗi!"))
        })
        .responder()
}

pub fn update_full_image((id, image, state): (Path<u32>, Json<CreateImage>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::UpdateBackground(image.into_inner(), id.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Background(background)) => Ok(HttpResponse::Ok().json(background)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Cập nhật bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Cập nhật bị lỗi!"))
        })
        .responder()
}


/// Manipulate with api for review
pub fn get_reviews(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::GetAllReviews)
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::AllReviews(reviews)) => Ok(HttpResponse::Ok().json(reviews)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống lấy danh sách đánh giá bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Lấy danh sách đánh giá bị lỗi")),
        })
        .responder()
}
pub fn get_full_reviews(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::GetAllFullReviews)
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::AllFullReviews(reviews)) => Ok(HttpResponse::Ok().json(reviews)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống lấy danh sách đánh giá bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Lấy danh sách đánh giá bị lỗi")),
        })
        .responder()
}
pub fn delete_review((id, state): (Path<u32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::DeleteReview(id.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Status(status)) => {
                if status == 1 {
                    Ok(HttpResponse::Ok().json("Xoá thành công!"))
                } else {
                    Ok(HttpResponse::BadRequest().json("Xoá thất bại!"))
                }
            },
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống xoá đánh giá bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Xoá đánh giá bị lỗi")),
        })
        .responder()
}


pub fn change_review_status((id, _status, state): (Path<u32>, Json<Status>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::ChangeReviewStatus(id.into_inner(), _status.into_inner().status))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Status(status)) => {
                if status == 1 {
                    Ok(HttpResponse::Ok().json("Ok!"))
                } else {
                    Ok(HttpResponse::BadRequest().json("Thất bại!"))
                }
            },
            Ok(_) => Ok(HttpResponse::BadRequest().json("Hệ thống thay đổi trạng thái đánh giá bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Thay đổi trạng thái đánh giá bị lỗi")),
        })
        .responder()
}


pub fn create_full_review((review, state): (Json<CreateReview>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::CreateReview(review.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Review(review)) => Ok(HttpResponse::Ok().json(review)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Tạo mới bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Tạo mới bị lỗi!"))
        })
        .responder()
}

pub fn update_full_review((id, review, state): (Path<u32>, Json<CreateReview>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(Queries::UpdateReview(review.into_inner(), id.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(ResponseTypes::Review(review)) => Ok(HttpResponse::Ok().json(review)),
            Ok(_) => Ok(HttpResponse::BadRequest().json("Cập nhật bị lỗi!")),
            Err(_err) => Ok(HttpResponse::BadRequest().json("Cập nhật bị lỗi!"))
        })
        .responder()
}

pub fn upload(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    Box::new(
        req.multipart()
            .map_err(error::ErrorInternalServerError)
            .map(handle_multipart_item)
            .flatten()
            .collect()
            .map(|sizes| HttpResponse::Ok().json(sizes))
            .map_err(|e| {
                println!("failed: {}", e);
                e
            }),
    )
}

pub fn login((auth_data, req): (Json<AuthData>, HttpRequest<AppState>))
    -> FutureResponse<HttpResponse> {
    req.state().db.send(Queries::AuthUser(auth_data.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(ResponseTypes::SlimUser(slim_user)) => {
                let token = create_token(&slim_user)?;
                req.remember(token.clone());
                Ok(HttpResponse::Ok().json(token))
            },
            Ok(_) => Ok(HttpResponse::InternalServerError().json("Lỗi hệ thống!")),
            Err(_err) => Ok(HttpResponse::InternalServerError().json("Lỗi hệ thống!")),
        }).responder()
}

pub fn logout(req: HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Ok().into()
}

pub fn get_user(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}