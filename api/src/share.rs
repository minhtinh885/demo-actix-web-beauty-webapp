use handler::{UserData};
use utils::hash_password;

pub fn init_user(database_url: String) -> () {
    let pool = mysql::Pool::new(database_url).unwrap();
    let result = pool.first_exec("SELECT COUNT(*) FROM users;", ()).unwrap().unwrap();
    let total: u32 = mysql::from_row(result);
    if total == 0 {
        let user = UserData {
            fullname: "Admin".into(),
            email: "admin@admin.com".into(),
            password: "matkhau".into(),
        };
        let stmt = "CALL create_user(:email, :fullname, :password, @status);";
        let password: String = hash_password(user.password.as_str()).expect("Lỗi tạo mật khẩu");

        pool.prep_exec(stmt, (user.email, user.fullname, password)).unwrap();

        pool.prep_exec("INSERT INTO role_user(role_id, user_id) VALUES (1, 1);", ()).unwrap();
    }

}
