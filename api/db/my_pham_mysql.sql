DROP DATABASE IF EXISTS my_pham;
CREATE DATABASE IF NOT EXISTS my_pham;

USE my_pham;

CREATE TABLE roles (
	id TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description VARCHAR(255)
);


CREATE TABLE users (
	id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    fullname VARCHAR(255) NOT NULL,
    password VARCHAR(64) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE role_user (
	role_id TINYINT UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (role_id) REFERENCES roles (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (role_id, user_id)
);

CREATE TABLE images (
	id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_url VARCHAR(255) NOT NULL,
    alt VARCHAR(255) NOT NULL,
    status TINYINT DEFAULT 0
);


CREATE TABLE reviews (
	id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    fullname VARCHAR(255) NOT NULL,
    gender TINYINT,
    phone_number VARCHAR(20),
    content VARCHAR(255),
    position_at_company VARCHAR(255),
    image_url VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW(),
    status TINYINT DEFAULT 0
);

CREATE TABLE subscribers (
	id VARCHAR(36) NOT NULL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    activated TINYINT DEFAULT 0
);


DELIMITER $$
CREATE PROCEDURE create_user(
	p_email VARCHAR(255), 
    p_fullname VARCHAR(255), 
    p_password VARCHAR(64),
    OUT status TINYINT)
BEGIN 
	DECLARE exist_id INT UNSIGNED DEFAULT 0;
    DECLARE EXIT HANDLER FOR SQLEXCEPTION SET status = 0;

	INSERT INTO users (email, fullname, password) VALUES (p_email, p_fullname, p_password);
	SET status = 1;

END$$

CREATE PROCEDURE create_review(
	p_fullname VARCHAR(255),
    p_gender TINYINT,
    p_phone_number VARCHAR(20),
    p_content VARCHAR(255),
    p_position_at_company VARCHAR(255),
    p_image_url VARCHAR(255),
    p_status TINYINT)
BEGIN
	INSERT INTO reviews(fullname, gender, phone_number, content, position_at_company, image_url, status)
    VALUES (p_fullname, p_gender, p_phone_number, p_content, p_position_at_company, p_image_url, p_status);
    SELECT id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status
    FROM reviews 
    WHERE id = LAST_INSERT_ID();
END$$

CREATE PROCEDURE update_review(
	p_id INT UNSIGNED,
	p_fullname VARCHAR(255),
    p_gender TINYINT,
    p_phone_number VARCHAR(20),
    p_content VARCHAR(255),
    p_position_at_company VARCHAR(255),
    p_image_url VARCHAR(255),
    p_status TINYINT)
BEGIN
	UPDATE reviews
    SET fullname = p_fullname, gender = p_gender, phone_number = p_phone_number, content = p_content, position_at_company = p_position_at_company, image_url = p_image_url, status = p_status
    WHERE id = p_id;
    
    SELECT id, fullname, gender, phone_number, content, position_at_company, image_url, created_at, status
    FROM reviews 
    WHERE id = p_id;
END$$

CREATE PROCEDURE create_image(
	p_image_url VARCHAR(255),
    p_alt VARCHAR(255),
    p_status TINYINT)
BEGIN
	INSERT INTO images(image_url, alt, status)
    VALUES (p_image_url, p_alt, p_status);
    
    SELECT id, image_url, alt, status
    FROM images 
    WHERE id = LAST_INSERT_ID();
END$$

CREATE PROCEDURE update_image(
	p_id INT UNSIGNED,
	p_image_url VARCHAR(255),
    p_alt VARCHAR(255),
    p_status TINYINT)
BEGIN
	UPDATE images
    SET image_url = p_image_url, alt = p_alt, status = p_status
    WHERE id = p_id;
    
    SELECT id, image_url, alt, status
    FROM images 
    WHERE id = p_id;
END$$

CREATE PROCEDURE list_roles(
	p_email VARCHAR(255))
BEGIN 
	SELECT roles.name 
    FROM users LEFT JOIN role_user 
		ON users.id = role_user.user_id
        LEFT JOIN roles
        ON role_user.role_id = roles.id
	WHERE p_email = users.email;
END$$
DELIMITER ;

INSERT INTO roles (name, description) 
	VALUES ('Admin', 'Quản trị viên có thể tạo tại khoản mới.'), 
		('Employee', 'Nhân viên có thể thay đổi mật khẩu hiện tại để thực hiện các tác vụ thường như nhập liệu');
        
-- INSERT INTO images (image_url, alt, status) 
-- VALUES ('background-1.png', 'Ảnh nền một', 1), 
-- 	('background-2.png', 'Ảnh nền hai', 1),
--    ('background-3.png', 'Ảnh nền ba', 1);

INSERT INTO reviews (fullname, gender, phone_number, content, position_at_company, image_url,status) 
VALUES ('Ryan Ho', 1, '0968.555.555', 'Đây là một sản phẩm đột phá của thế kỷ 21. Tôi đã sử dụng và hiệu quả trông thấy chỉ sau 1 tháng. Thật tuyệt vời!', 'Head of Marketing', 'customer.jpg',1);
INSERT INTO reviews (fullname, gender, phone_number, content, position_at_company, image_url,status) 
VALUES ('Ryan Tan', 1, '0968.555.556', 'Đây là một sản phẩm đột phá của thế kỷ 21. Tôi đã sử dụng và hiệu quả trông thấy chỉ sau 1 tháng. Thật tuyệt vời!', 'Head of Marketing', 'customer.jpg',1);

-- SELECT * FROM users;
-- SELECT * FROM roles;
-- SELECT * FROM reviews;
-- INSERT INTO role_user(role_id, user_id) VALUES (1, 1);
-- CALL list_roles('admin1@example.com');
