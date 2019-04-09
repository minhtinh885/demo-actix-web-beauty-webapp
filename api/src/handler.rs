use actix::{Handler, Message};
use actix_web::*;
use db::DbExecutor;
use db::Connection;
use utils::{hash_password, decode_token};
use errors::ServiceError;
use bcrypt::verify;
use actix_web::middleware::identity::RequestIdentity;
use chrono::NaiveDateTime;

use std::fs;
use std::io::Write;

use futures::future;
use futures::{Future, Stream};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub fullname: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: u32,
    pub image_url: String,
    pub alt: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FullImage {
    pub id: u32,
    pub image_url: String,
    pub alt: String,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImage {
    pub image_url: String,
    pub alt: String,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub id: u32,
    pub fullname: String,
    pub phone_number: String,
    pub content: String,
    pub position_at_company: String,
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReview {
    pub fullname: String,
    pub gender: u32,
    pub phone_number: String,
    pub position_at_company: String,
    pub status: u32,
    pub content: String,
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullReview {
    pub id: u32,
    pub fullname: String,
    pub gender: u8,
    pub phone_number: String,
    pub content: String,
    pub position_at_company: String,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub enum ResponseTypes {
    Roles(Vec<String>),
    Status(u8),
    AllImages(Vec<Image>),
    Background(FullImage),
    Backgrounds(Vec<FullImage>),
    AllReviews(Vec<Review>),
    Review(FullReview),
    AllFullReviews(Vec<FullReview>),
    SlimUser(SlimUser),

}

pub enum Queries {
    ListRoles(String),
    AuthUser(AuthData),
    RegisterUser(UserData),

    /// Image - Backgrounds
    GetAllImages,
    CreateBackground(CreateImage),
    UpdateBackground(CreateImage, u32),
    GetAllBackgrounds,
    DeleteBackground(u32),
    ChangeBackgroundStatus(u32, u8),

    /// REVIEW
    GetAllReviews,
    CreateReview(CreateReview),
    UpdateReview(CreateReview, u32),
    GetAllFullReviews,
    DeleteReview(u32),
    ChangeReviewStatus(u32, u8),
}

impl Message for Queries {
    type Result = Result<ResponseTypes, ServiceError>;
}

impl Handler<Queries> for DbExecutor {
    type Result = Result<ResponseTypes, ServiceError>;

    fn handle(&mut self, msg: Queries, _ctx: &mut Self::Context) -> Self::Result {
        let conn: Connection = self.0.get().map_err(|_err| {
            return ServiceError::InternalServerError;
        }).unwrap();

        match msg {
            Queries::ListRoles(email) => list_roles(conn, email),
            Queries::AuthUser(auth_data) => auth_user(conn, auth_data),
            Queries::RegisterUser(user_data) => register(conn, user_data),

            // Image - Backgrounds
            Queries::GetAllImages => get_all_images(conn),
            Queries::GetAllBackgrounds => get_all_backgrounds(conn),
            Queries::DeleteBackground(id) => delete_background(conn, id),
            Queries::ChangeBackgroundStatus(id, image_status) => change_background_status(conn, id, image_status),
            Queries::CreateBackground(new_image) => create_background(conn, new_image),
            Queries::UpdateBackground(image, id) => update_background(conn, image, id),

            // REVIEW
            Queries::GetAllReviews => get_all_reviews(conn),
            Queries::GetAllFullReviews => get_all_full_reviews(conn),
            Queries::DeleteReview(id) => delete_review(conn, id),
            Queries::ChangeReviewStatus(id, review_status) => change_review_status(conn, id, review_status),
            Queries::CreateReview(new_review) => create_review(conn, new_review),
            Queries::UpdateReview(review, id) => update_review(conn, review, id),
        }
    }

}

pub fn list_roles(mut conn: Connection, email: String) -> Result<ResponseTypes, ServiceError> {
    let stmt = format!("CALL list_roles('{}');", email);
    let roles: Vec<String> = conn.query(stmt)
        .map(|result| result.map(|x| x.unwrap())
        .filter_map(|row| {
            let result: Option<String> = mysql::from_row(row);
            result
        }).collect()
    ).unwrap();
    Ok(ResponseTypes::Roles(roles))
}

pub fn auth_user(mut conn: Connection, auth_data: AuthData) -> Result<ResponseTypes, ServiceError> {
    let query = format!("SELECT email, fullname, password FROM users WHERE email = '{}'", auth_data.email);
    let mismatch_error = Err(ServiceError::BadRequest("Tên đăng nhập và mật khẩu không đúng!".into()));

    let mut users: Vec<UserData> = conn.query(query)
        .map(|result| result.map(|x| x.unwrap())
            .map(|row| {
                let (email, fullname, password) = mysql::from_row(row);
                UserData {email, fullname, password}
            }).collect()
        ).unwrap();

    if let Some(user) = users.pop() {
        match verify(&auth_data.password, &user.password) {
            Ok(matching) => {
                if matching {
                    return Ok(ResponseTypes::SlimUser(SlimUser{email: user.email}));
                } else {
                    return mismatch_error;
                }
            },
            Err(_) => {return mismatch_error;},
        }
    }
    mismatch_error

}


/// Dùng để đăng ký người dùng mới
/// Trả về kết quả 0 là tài khoản email đã tồn tại
/// Trả về kết quả khách 0 là t
pub fn register (mut conn: Connection, user_data: UserData) -> Result<ResponseTypes, ServiceError> {
    let query = format!("SELECT email FROM users WHERE email = '{}';", user_data.email);
    let email: Option<String> = conn.first(query).unwrap();

    match email {
        Some(_) => Ok(ResponseTypes::Status( 0 )),
        None => {
            let stmt = "CALL create_user(:email, :fullname, :password, @status);";
            let password: String = hash_password(user_data.password.as_str())?;
            conn.prep_exec(stmt, (user_data.email, user_data.fullname, password))
                .unwrap();
            let status: u8 = match conn.first("SELECT @status;").unwrap() {
                Some(val) => val,
                None => 0,
            };
            Ok(ResponseTypes::Status( status ))
        }
    }

}

pub fn get_all_images(mut conn: Connection) -> Result<ResponseTypes, ServiceError> {
    let images = conn.query("SELECT id, image_url, alt FROM images WHERE status <> 0")
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, image_url, alt) = mysql::from_row(row);
                    Image {id, image_url, alt}
                }).collect()
        }).unwrap();

    Ok(ResponseTypes::AllImages(images))
}

pub fn get_all_backgrounds(mut conn: Connection) -> Result<ResponseTypes, ServiceError> {
    let reviews = conn.query("SELECT id, image_url, alt, status FROM images")
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, image_url, alt, status) = mysql::from_row(row);
                    FullImage {id, image_url, alt, status}
                }).collect()
        }).unwrap();

    Ok(ResponseTypes::Backgrounds(reviews))
}

pub fn delete_background(mut conn: Connection, id: u32) -> Result<ResponseTypes, ServiceError> {
    let query = format!("DELETE FROM images WHERE id = {};", id);
    let mut status: u8 = 1;
    if let Err(_err) = conn.query(query) {
        status = 0
    }
    Ok(ResponseTypes::Status(status))
}

pub fn change_background_status(mut conn: Connection, id: u32, image_status: u8) -> Result<ResponseTypes, ServiceError> {
    let query = format!("UPDATE images SET status = {} WHERE id = {}", image_status, id);
    let mut status: u8 = 1;
    if let Err(_err) = conn.query(query) {
        status = 0;
    }
    Ok(ResponseTypes::Status(status))
}

pub fn create_background(mut conn: Connection, new_image: CreateImage) -> Result<ResponseTypes, ServiceError> {
    let mut images: Vec<FullImage> = conn.prep_exec("CALL create_image(?, ?, ?);",
                                                      (new_image.image_url, new_image.alt, new_image.status))
        .map_err(|_e| ServiceError::BadRequest("Không thể tạo hình ảnh".into()))
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, image_url, alt, status) = mysql::from_row(row);
                    FullImage {id, image_url, alt, status}
                }).collect()
        }).unwrap();
    Ok(ResponseTypes::Background(images.pop().unwrap()))
}

pub fn update_background(mut conn: Connection, new_image: CreateImage, id: u32) -> Result<ResponseTypes, ServiceError> {
    let mut images: Vec<FullImage> = conn.prep_exec("CALL update_image(?, ?, ?, ?);",
                                                    (id, new_image.image_url, new_image.alt, new_image.status))
        .map_err(|_e| ServiceError::BadRequest("Không thể tạo cập nhật ảnh".into()))
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, image_url, alt, status) = mysql::from_row(row);
                    FullImage {id, image_url, alt, status}
                }).collect()
        }).unwrap();
    Ok(ResponseTypes::Background(images.pop().unwrap()))
}

/// Manipulate with reviews
///
pub fn get_all_reviews(mut conn: Connection) -> Result<ResponseTypes, ServiceError> {
    let reviews = conn.query("SELECT id, fullname, phone_number, content, position_at_company, image_url FROM reviews WHERE status <> 0")
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, fullname, phone_number, content, position_at_company, image_url) = mysql::from_row(row);
                    Review {id, fullname, phone_number, content, position_at_company, image_url}
                }).collect()
        }).unwrap();

    Ok(ResponseTypes::AllReviews(reviews))
}

pub fn get_all_full_reviews(mut conn: Connection) -> Result<ResponseTypes, ServiceError> {
    let reviews = conn.query("SELECT id, gender, fullname, phone_number, content, position_at_company, image_url, created_at, status FROM reviews")
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, gender, fullname, phone_number, content, position_at_company, image_url, created_at, status) = mysql::from_row(row);
                    FullReview {id, gender, fullname, phone_number, content, position_at_company, image_url, created_at, status}
                }).collect()
        }).unwrap();

    Ok(ResponseTypes::AllFullReviews(reviews))
}

pub fn delete_review(mut conn: Connection, id: u32) -> Result<ResponseTypes, ServiceError> {
    let query = format!("DELETE FROM reviews WHERE id = {};", id);
    let mut status: u8 = 1;
    if let Err(_err) = conn.query(query) {
        status = 0
    }
    Ok(ResponseTypes::Status(status))
}

pub fn change_review_status(mut conn: Connection, id: u32, review_status: u8) -> Result<ResponseTypes, ServiceError> {
    let query = format!("UPDATE reviews SET status = {} WHERE id = {}", review_status, id);
    let mut status: u8 = 1;
    if let Err(_err) = conn.query(query) {
        status = 0;
    }
    Ok(ResponseTypes::Status(status))
}

pub fn create_review(mut conn: Connection, new_review: CreateReview) -> Result<ResponseTypes, ServiceError> {
    let mut reviews: Vec<FullReview> = conn.prep_exec("CALL create_review(?, ?, ?, ?, ?, ?, ?);",
                              (new_review.fullname, new_review.gender, new_review.phone_number, new_review.content,
                               new_review.position_at_company, new_review.image_url, new_review.status))
        .map_err(|_e| ServiceError::BadRequest("Không thể tạo review".into()))
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status) = mysql::from_row(row);
                    FullReview {id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status}
                }).collect()
        }).unwrap();
    Ok(ResponseTypes::Review(reviews.pop().unwrap()))
}

pub fn update_review(mut conn: Connection, review: CreateReview, id: u32) -> Result<ResponseTypes, ServiceError> {
    let mut reviews: Vec<FullReview> = conn.prep_exec("CALL update_review(?, ?, ?, ?, ?, ?, ?, ?);",
                                                      (id, review.fullname, review.gender, review.phone_number, review.content,
                                                       review.position_at_company, review.image_url, review.status))
        .map_err(|_e| ServiceError::BadRequest("Không thể cập nhật review".into()))
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status) = mysql::from_row(row);
                    FullReview {id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status}
                }).collect()
        }).unwrap();
    Ok(ResponseTypes::Review(reviews.pop().unwrap()))
}

// we need the same data as SlimUser
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;
impl<S> FromRequest<S> for LoggedUser {
    type Config = ();
    type Result = Result<LoggedUser, ServiceError>;

    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        if let Some(identity) = req.identity() {
            let user: SlimUser = decode_token(&identity)
                .map_err(|_err| ServiceError::Unauthorized)
                .unwrap();
            return Ok(user as LoggedUser);
        }
        Err(ServiceError::Unauthorized)
    }
}


// UPLOAD FILE
pub fn save_file(
    field: multipart::Field<dev::Payload>,
) -> Box<Future<Item = i64, Error = Error>> {

    let mut file_path_string = String::from("../webapp/static/img/");

    match field.content_disposition() {
        Some(data) => {
            for parameter in data.parameters.iter() {
                if let Some(filename) = parameter.as_filename() {
                    let s = String::from(filename);
                    file_path_string.push_str(&s);
                    /*if let Some(num) = s.find(r".") {
                        file_path_string.push_str(&s[num..]);
                    }*/
                }
            }
        },
        None => {
            let rand_string: String = ::std::iter::repeat(())
                .map(|()| rand::thread_rng().sample(rand::distributions::Alphanumeric))
                .take(10)
                .collect();
            file_path_string.push_str(rand_string.as_str());
            file_path_string.push_str(".");
            file_path_string.push_str(field.content_type().subtype().as_str());
        }
    }
    let mut file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Box::new(future::err(error::ErrorInternalServerError(e))),
    };
    Box::new(
        field
            .fold(0i64, move |acc, bytes| {
                let rt = file
                    .write_all(bytes.as_ref())
                    .map(|_| acc + bytes.len() as i64)
                    .map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        error::MultipartError::Payload(error::PayloadError::Io(e))
                    });
                future::result(rt)
            })
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn handle_multipart_item(
    item: multipart::MultipartItem<dev::Payload>,
) -> Box<Stream<Item = i64, Error = Error>> {
    match item {
        multipart::MultipartItem::Field(field) => {
            Box::new(save_file(field).into_stream())
        }
        multipart::MultipartItem::Nested(mp) => Box::new(
            mp.map_err(error::ErrorInternalServerError)
                .map(handle_multipart_item)
                .flatten(),
        ),
    }
}