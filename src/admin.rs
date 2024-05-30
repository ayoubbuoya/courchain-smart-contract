use crate::models::*;
use crate::{Contract, ContractExt};
use near_sdk::env::log_str;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    pub fn change_user_to_admin(&mut self, account_id: AccountId) {
        // check if caller is admin
        let account_id = env::signer_account_id();

        // check if the caller account is ayoubamer.testnet
        assert!(
            account_id.to_string() == "ayoubamer.testnet",
            "Only admin can set admins"
        );

        // check if the user exists
        let user = self.get_user_by_id(account_id.clone());
        if user.is_none() {
            log_str("User does not exist");
            env::panic_str("User does not exist")
        }

        let mut user = user.unwrap();
        user.role = "admin".to_string();

        // getting the user index in the users vector
        let user_index = self
            .users
            .iter()
            .position(|u| u.account_id == user.account_id)
            .unwrap();

        // update the user
        self.users.replace(user_index as u32, user.clone());

        log_str(&format!("User {} is now an admin", account_id));
    }

    pub fn update_course_price_by_admin(&mut self, course_id: u64, price: U128) {
        // check if caller is admin
        let account_id = env::signer_account_id();

        // check if the caller account is ayoubamer.testnet
        assert!(
            account_id.to_string() == "ayoubamer.testnet",
            "Only admin can update course price"
        );

        let mut course = self.get_course_by_id(course_id.clone()).unwrap();
        // check if course exists
        assert!(course.id == course_id, "Course does not exist");
        course.price = u128::from(price);

        self.courses.set(course_id as u32, course);
    }

    pub fn save_course_by_admin(
        &mut self,
        mentor_id: AccountId,
        title: String,
        description: String,
        level: String,
        duration: String,
        category: String,
        requirements: Vec<String>,
        objectives: Vec<String>,
        picture: String,
        with_ai: bool,
        price: u128,
        created_at: u64,
    ) -> u64 {
        let account_id: AccountId = env::signer_account_id();

        // only ayoubamer.testnet can save courses
        if account_id.to_string() != "ayoubamer.testnet".to_string() {
            // return Err("Only the admin can save courses".to_string());
            env::panic_str("Only the admin can save courses")
        }

        // check if the mentor exists
        // let mentor = self.get_user_by_id(mentor_id.clone());
        // if mentor.is_none() {
        //     log_str("Mentor does not exist");
        //     env::panic_str("Mentor does not exist")
        // }

        log_str(&format!("Creating New Course..."));
        let new_course: Course = Course {
            id: self.course_count,
            title: title.clone(),
            description,
            level,
            duration,
            status: "draft".to_string(),
            category,
            requirements,
            objectives,
            picture,
            with_ai,
            price,
            created_at,
            updated_at: created_at,
            mentor_id: mentor_id.clone(),
            modules_ids: vec![],
        };

        // add course to the courses vector
        self.courses.push(new_course.clone());

        // Increment the course count
        self.course_count += 1;

        // Log the creation of the course
        log_str(&format!("Course created: {} by {}", title, mentor_id));

        return new_course.id;
    }

    pub fn save_module_by_admin(
        &mut self,
        course_id: u64,
        title: String,
        description: String,
        status: String,
        order: u64,
        with_ai: bool,
        created_at: u64,
    ) -> Module {
        let account_id: AccountId = env::signer_account_id();

        // only ayoubamer.testnet can save modules
        if account_id.to_string() != "ayoubamer.testnet".to_string() {
            env::panic_str("Only the admin can save modules")
        }

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            env::panic_str("Course does not exist")
        }

        log_str(&format!("Creating New Module..."));

        let new_module: Module = Module {
            id: self.module_count,
            title: title.clone(),
            course_id,
            description,
            status,
            order,
            with_ai,
            created_at,
            updated_at: created_at,
            lessons_ids: vec![],
            quizz_id: None,
        };

        // add module to the modules vector
        self.modules.push(new_module.clone());

        // add the module id to the course
        let mut course = course.unwrap();
        course.modules_ids.push(self.module_count);

        // update the course
        self.courses.replace(course_id as u32, course.clone());

        // Log the creation of the module
        log_str(&format!("Module created: {} by {}", title, account_id));

        // Increment the module count
        self.module_count += 1;

        return new_module;
    }

    pub fn save_lesson_by_admin(
        &mut self,
        module_id: u64,
        title: String,
        description: String,
        video_url: String,
        article: String,
        order: u64,
        with_ai: bool,
        created_at: u64,
    ) {
        let account_id: AccountId = env::signer_account_id();

        // only ayoubamer.testnet can save lessons
        if account_id.to_string() != "ayoubamer.testnet".to_string() {
            env::panic_str("Only the admin can save lessons")
        }

        // check if the module exists
        let module = self.get_module_by_id(module_id);
        if module.is_none() {
            log_str("Module does not exist");
            env::panic_str("Module does not exist")
        }

        log_str(&format!("Creating New Lesson..."));

        let new_lesson: Lesson = Lesson {
            id: self.lesson_count,
            title: title.clone(),
            module_id,
            description,
            video_url,
            article,
            order,
            with_ai,
            created_at,
            updated_at: created_at,
        };

        // add lesson to the lessons vector
        self.lessons.push(new_lesson);

        // Log the creation of the lesson
        log_str(&format!("Lesson created: {} by {}", title, account_id));

        // add the lesson id to the module
        let mut module = module.unwrap();
        module.lessons_ids.push(self.lesson_count);

        // update the module
        self.modules.replace(module_id as u32, module.clone());

        // Log the addition of the lesson to the module
        log_str(&format!("Lesson added to module: {}", module.title));

        // Increment the lesson count
        self.lesson_count += 1;
    }

    pub fn update_lesson_by_admin(
        &mut self,
        lesson_id: u64,
        title: String,
        description: String,
        video_url: String,
        article: String,
        order: u64,
        with_ai: bool,
        updated_at: u64,
    ) -> Lesson {
        let account_id: AccountId = env::signer_account_id();

        // only ayoubamer.testnet can update lessons
        if account_id.to_string() != "ayoubamer.testnet".to_string() {
            env::panic_str("Only the admin can update lessons")
        }

        // get the lesson
        let lesson = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            env::panic_str("Lesson does not exist")
        }

        log_str(&format!("Updating Lesson..."));

        // update the lesson
        let mut lesson = lesson.unwrap();
        lesson.title = title.clone();
        lesson.description = description;
        lesson.video_url = video_url;
        lesson.article = article;
        lesson.order = order;
        lesson.with_ai = with_ai;
        lesson.updated_at = updated_at;

        // update the lesson
        self.lessons.replace(lesson_id as u32, lesson.clone());

        // Log the update of the lesson
        log_str(&format!("Lesson updated: {}", title));

        return lesson;
    }

    pub fn update_user_min_info_by_admin(
        &mut self,
        account_id: AccountId,
        name: String,
        email: String,
        username: String,
        phone: String,
        updated_at: u64,
    ) -> User {
        let admin_account_id: AccountId = env::signer_account_id();

        // only ayoubamer.testnet can update users
        if admin_account_id.to_string() != "ayoubamer.testnet".to_string() {
            env::panic_str("Only the admin can update users")
        }

        // get the user
        let user = self.get_user_by_id(account_id.clone());
        if user.is_none() {
            log_str("User does not exist");
            env::panic_str("User does not exist")
        }

        log_str(&format!("Updating User..."));

        // update the user
        let mut user = user.unwrap();
        user.name = name.clone();
        user.email = email.clone();
        user.username = username.clone();
        user.phone = phone.clone();
        user.updated_at = updated_at;

        // find the user index in the users vector
        let user_index = self
            .users
            .iter()
            .position(|u| u.account_id == user.account_id)
            .unwrap();

        // update the user
        self.users.replace(user_index as u32, user.clone());

        // Log the update of the user
        log_str(&format!("User updated : {}", account_id));

        return user;
    }
}
