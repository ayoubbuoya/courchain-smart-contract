use crate::{Contract, ContractExt, Course};
use near_sdk::{env, log, near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    pub fn hello_msg(&self) -> String {
        "Hello, World!".to_string()
    }

    pub fn current_user(&self) -> AccountId {
        let account_id: AccountId = env::signer_account_id();
        account_id
    }

    pub fn user_exists_by_account_id(&self, account_id: AccountId) -> bool {
        for user in self.users.iter() {
            if user.account_id == account_id {
                return true;
            }
        }
        false
    }

    pub fn user_exists(&self, account_id: AccountId, username: String, email: String) -> bool {
        for user in self.users.iter() {
            if user.account_id == account_id || user.username == username || user.email == email {
                return true;
            }
        }
        false
    }

    pub fn is_admin(&self, account_id: AccountId) -> bool {
        let user = self.get_user_by_id(account_id);
        if let Some(user) = user {
            if user.role == "admin" {
                return true;
            }
        }
        false
    }

    pub fn is_course_mentor(&self, course_id: u64, account_id: AccountId) -> bool {
        let course = self.get_course_by_id(course_id);
        if course.unwrap().mentor_id == account_id {
            return true;
        }
        false
    }

    pub fn is_student_course_carted(&self, course_id: u64, account_id: AccountId) -> bool {
        let enrollment = self.get_enrollment(course_id, account_id);
        if let Some(enrollment) = enrollment {
            if enrollment.status == "carted" {
                return true;
            }
        }
        false
    }

    pub fn is_student_course_enrolled(&self, course_id: u64, account_id: AccountId) -> bool {
        let enrollment = self.get_enrollment(course_id, account_id);
        if let Some(enrollment) = enrollment {
            if enrollment.status == "enrolled" {
                return true;
            }
        }
        false
    }

    pub fn is_student_course_completed(&self, course_id: u64, account_id: AccountId) -> bool {
        let enrollment = self.get_enrollment(course_id, account_id);
        if let Some(enrollment) = enrollment {
            if enrollment.status == "completed" {
                return true;
            }
        }
        false
    }

    pub fn is_student_module_enrolled(&self, module_id: u64, account_id: AccountId) -> bool {
        let module_progress = self.get_module_progress(module_id, account_id);
        if let Some(module_progress) = module_progress {
            if module_progress.is_enrolled == true {
                return true;
            }
        }
        false
    }

    pub fn is_student_lesson_enrolled(&self, lesson_id: u64, account_id: AccountId) -> bool {
        let lesson_progress = self.get_lesson_progress(lesson_id, account_id);
        if let Some(lesson_progress) = lesson_progress {
            if lesson_progress.is_enrolled == true {
                return true;
            }
        }
        false
    }

    pub fn is_student_lesson_completed(&self, lesson_id: u64, account_id: AccountId) -> bool {
        let lesson_progress = self.get_lesson_progress(lesson_id, account_id);
        if let Some(lesson_progress) = lesson_progress {
            if lesson_progress.status == "completed" {
                return true;
            }
        }
        false
    }

    pub fn is_student_module_completed(&self, module_id: u64, account_id: AccountId) -> bool {
        let module = self.get_module_by_id(module_id);
        if let Some(module) = module {
            let lessons = module.lessons_ids;
            for lesson_id in lessons.iter() {
                let lesson_progress = self.get_lesson_progress(*lesson_id, account_id.clone());
                if let Some(lesson_progress) = lesson_progress {
                    if lesson_progress.status != "completed" {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn calculate_total_courses_price_with_fee(&self, courses: Vec<Course>) -> u128 {
        let mut total_price: u128 = 0;
        let fee_percentage: u128 = 10;

        for course in courses.iter() {
            // add teh fee to each course
            let course_price = course.price;
            log!("Course Price: {}", course_price);
            let course_fee = course_price * fee_percentage / 100;
            log!("Course Fee: {}", course_fee);
            let course_total_price = course_price + course_fee;
            log!("Course Total Price: {}", course_total_price);
            total_price += course_total_price;
        }
        total_price
    }

   

    /* pub fn calculate_module_creation_progress(&self, module_id: u64) -> u64 {
        let module = self.get_module_by_id(module_id).unwrap();
        let lessons = module.lessons_ids;
        let mut completed_lessons = 0;
        for lesson_id in lessons.iter() {
            // check if the lesson has artcile or video url content
            let lesson = self.get_lesson_by_id(*lesson_id).unwrap();
            if !lesson.article.is_empty() || !lesson.video_url.is_empty() {
                completed_lessons += 1;
            }
        }

        let total_lessons = lessons.len() as u64;
        if total_lessons == 0 {
            return 0;
        }
        let progress = (completed_lessons * 100) / total_lessons;
        progress
    }

    pub fn calculate_course_creation_progress(&self, course: Course) -> u64 {
        let modules = course.modules_ids;
        let total_modules = modules.len() as u64;
        if total_modules == 0 {
            return 0; 
        }
        let mut completed_modules = 0;
        for module_id in modules.iter() {
            let progress = self.calculate_module_creation_progress(*module_id);
            if progress == 100 {
                completed_modules += 1;
            }
        }
        let progress = (completed_modules * 100) / total_modules;
        progress
    } */
}
