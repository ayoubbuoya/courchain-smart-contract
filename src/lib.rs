use models::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{self, log_str};
use near_sdk::json_types::U128;
use near_sdk::store::Vector;
use near_sdk::{log, near_bindgen, AccountId, Promise};

mod admin;
mod getters;
mod models;
mod unit_tests;
mod utils;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub users: Vector<User>,
    pub courses: Vector<Course>,
    pub modules: Vector<Module>,
    pub lessons: Vector<Lesson>,
    pub enrollments: Vector<Enrollment>,
    pub quizzes: Vector<Quizz>,
    pub module_progresses: Vector<ModuleProgress>,
    pub lesson_progresses: Vector<LessonProgress>,
    pub quizz_progresses: Vector<QuizzProgress>,
    pub user_count: u64,
    pub course_count: u64,
    pub module_count: u64,
    pub lesson_count: u64,
    pub assignment_count: u64,
    pub enrollment_count: u64,
    pub module_progress_count: u64,
    pub lesson_progress_count: u64,
    pub quizz_progress_count: u64,
    pub quizz_count: u64,
    pub test_count: u64,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: env::current_account_id(),
            users: Vector::new(b"u".to_vec()),
            courses: Vector::new(b"c".to_vec()),
            modules: Vector::new(b"m".to_vec()),
            lessons: Vector::new(b"l".to_vec()),
            enrollments: Vector::new(b"e".to_vec()),
            quizzes: Vector::new(b"q".to_vec()),
            module_progresses: Vector::new(b"mp".to_vec()),
            lesson_progresses: Vector::new(b"lp".to_vec()),
            quizz_progresses: Vector::new(b"qp".to_vec()),
            user_count: 0,
            course_count: 0,
            module_count: 0,
            lesson_count: 0,
            quizz_count: 0,
            assignment_count: 0,
            enrollment_count: 0,
            module_progress_count: 0,
            lesson_progress_count: 0,
            quizz_progress_count: 0,

            test_count: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn set_test_count(&mut self, count: u64) {
        self.test_count = count;
        log!("Tzst Count Updated")
    }

    pub fn get_test_count(&self) -> u64 {
        self.test_count
    }

    pub fn create_user(
        &mut self,
        name: String,
        username: String,
        phone: String,
        email: String,
        by_google: bool,

        picture: String,
        created_at: u64,
    ) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // Check if the user already exists
        if self.user_exists(account_id.clone(), username.clone(), email.clone()) {
            return false;
        }

        log_str(&format!("Creating New User..."));

        let new_user: User = User {
            account_id,
            name,
            username: username.clone(),
            email,
            phone,
            role: "user".to_string(), // default role is "user

            by_google,

            certifications: None,

            picture,
            created_at,
            updated_at: created_at,
        };

        // add user to the users vector
        self.users.push(new_user);

        // Increment the user count
        self.user_count += 1;

        // Log the creation of the user
        log_str(&format!("User created: {}", username));

        true
    }

    // update user
    pub fn update_user(
        &mut self,
        account_id: AccountId,
        name: String,
        username: String,
        phone: String,
        email: String,

        by_google: bool,

        picture: String,
        updated_at: u64,
    ) -> bool {
        let current_user = env::signer_account_id();

        // check if the user exists
        let user = self.get_user_by_id(account_id.clone());
        if user.is_none() {
            log_str("User does not exist");
            return false;
        }

        // check if the user is the current user or an admin
        if current_user != account_id.clone() && !self.is_admin(current_user.clone()) {
            log_str("Only the user or admin can update the profile");
            return false;
        }

        log_str(&format!("Updating User..."));

        // update the user
        let mut user = user.unwrap();
        user.name = name;
        user.username = username.clone();
        user.email = email;
        user.phone = phone;

        user.by_google = by_google;

        user.picture = picture;
        user.updated_at = updated_at;

        // get the index of the user in the users vector
        let index = self
            .users
            .iter()
            .position(|u| u.account_id == account_id)
            .unwrap();

        // update the user
        self.users.replace(index as u32, user.clone());

        // Log the update of the user
        log_str(&format!("User updated: {}", username));

        true
    }


    pub fn update_user_info(
        &mut self,
        name: String,
        phone: String,
        picture: String,
        updated_at: u64,
    ) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // check if the user exists
        let user = self.get_user_by_id(account_id.clone());
        if user.is_none() {
            log_str("User does not exist");
            return false;
        }

        log_str(&format!("Updating User Info..."));

        // update the user
        let mut user = user.unwrap();
        user.name = name.clone();
        user.phone = phone;
        user.picture = picture;
        user.updated_at = updated_at;

        // get the index of the user in the users vector
        let index = self
            .users
            .iter()
            .position(|u| u.account_id == account_id)
            .unwrap();

        // update the user
        self.users.replace(index as u32, user.clone());

        // Log the update of the user info
        log_str(&format!("User info updated: {}", name));

        true
    }



    // create course
    pub fn create_course(
        &mut self,
        title: String,
        description: String,
        level: String,
        duration: String,
        category: String,
        requirements: Vec<String>,
        objectives: Vec<String>,
        picture: String,
        with_ai: bool,
        price: U128,
        created_at: u64,
    ) -> Course {
        let account_id: AccountId = env::signer_account_id();

        // check if the mentor exists
        let mentor = self.get_user_by_id(account_id.clone());
        if mentor.is_none() {
            log_str("Mentor does not exist");
            env::panic_str("Mentor does not exist")
        }

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
            price: u128::from(price),
            created_at,
            updated_at: created_at,
            mentor_id: account_id.clone(),
            modules_ids: vec![],
        };

        // add course to the courses vector
        self.courses.push(new_course.clone());

        // Increment the course count
        self.course_count += 1;

        // Log the creation of the course
        log_str(&format!("Course created: {} by {}", title, account_id));

        return new_course;
    }

    pub fn create_module(
        &mut self,
        course_id: u64,
        title: String,
        description: String,
        status: String,
        order: u64,
        with_ai: bool,
        created_at: u64,
    ) -> bool {
        // only mentors can create modules
        let account_id: AccountId = env::signer_account_id();

        // check if the mentor exists
        let mentor = self.get_user_by_id(account_id.clone());
        if mentor.is_none() {
            log_str("Mentor does not exist");
            return false;
        }

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
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
        self.modules.push(new_module);

        // Log the creation of the module
        log_str(&format!("Module created: {} by {}", title, account_id));

        // add the module id to the course
        let mut course = course.unwrap();
        course.modules_ids.push(self.module_count);

        // update the course
        self.courses.replace(course_id as u32, course.clone());

        // Log the addition of the module to the course
        log_str(&format!("Module added to course: {}", course.title));

        // Increment the module count
        self.module_count += 1;

        return true;
    }

    pub fn update_module(
        &mut self,
        module_id: u64,
        title: String,
        description: String,
        updated_at: u64,
    ) {
        // Only the mentor of the module can update the module details
        let account_id: AccountId = env::signer_account_id();

        // check if the module exists
        let module: Option<Module> = self.get_module_by_id(module_id);
        if module.is_none() {
            log_str("Module does not exist");
            env::panic_str("Module does not exist")
        }

        // check if the mentor is the mentor of the module
        let module: Module = module.unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the module can update the module details");
            env::panic_str("Only the mentor of the module can update the module details")
        }

        // update the module details
        let mut module: Module = module.clone();
        module.title = title;
        module.description = description;
        module.updated_at = updated_at;

        // get the index of the module in the modules vector
        let index: usize = self.modules.iter().position(|m| m.id == module.id).unwrap();

        // update the module
        self.modules.replace(index as u32, module.clone());

        // Log the update of the module details
        log_str(&format!("Module details updated: {}", module.title));
    }

    pub fn create_quizz(
        &mut self,
        module_id: u64,
        title: String,
        description: String,
        created_at: u64,
    ) -> bool {
        // only mentors can create quizzes
        let account_id: AccountId = env::signer_account_id();

        // check if the module exists
        let module: Option<Module> = self.get_module_by_id(module_id.clone());
        if module.is_none() {
            log_str("Module does not exist");
            return false;
        }

        log_str(&format!("Creating New Quizz..."));

        let new_quizz: Quizz = Quizz {
            id: self.quizz_count,
            title: title.clone(),
            module_id,
            description,
            questions: vec![],
            with_ai: false,
            created_at,
            updated_at: created_at,
        };

        // add quizz to the quizzes vector
        self.quizzes.push(new_quizz);

        // Log the creation of the quizz
        log_str(&format!("Quizz created: {} by {}", title, account_id));

        // add the quizz id to the module
        let mut module = module.unwrap();
        module.quizz_id = Some(self.quizz_count);

        // update the module
        self.modules.replace(module_id as u32, module.clone());

        // Log the addition of the quizz to the module
        log_str(&format!("Quizz added to module: {}", module.title));

        // Increment the quizz count
        self.quizz_count += 1;

        return true;
    }

    pub fn save_quizz_questions(&mut self, quizz_id: u64, with_ai: bool, questions: Vec<Question>) {
        // only mentors can save quizz questions
        let account_id: AccountId = env::signer_account_id();

        // check if the quizz exists
        let quizz: Option<Quizz> = self.get_quizz_by_id(quizz_id);
        if quizz.is_none() {
            log_str("Quizz does not exist");
            env::panic_str("Quizz does not exist")
        }

        // check if the mentor is the mentor of the quizz
        let quizz: Quizz = quizz.unwrap();
        let module: Module = self.get_module_by_id(quizz.module_id).unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the quizz can save quizz questions");
            env::panic_str("Only the mentor of the quizz can save quizz questions")
        }

        // update the quizz questions
        let mut quizz: Quizz = quizz.clone();
        quizz.questions = questions;
        quizz.with_ai = with_ai;

        // get the index of the quizz in the quizzes vector
        let index: usize = self.quizzes.iter().position(|q| q.id == quizz.id).unwrap();

        // update the quizz
        self.quizzes.replace(index as u32, quizz.clone());

        // Log the saving of the quizz questions
        log_str(&format!("Quizz questions saved: {}", quizz.title));
    }

    pub fn create_lesson(
        &mut self,
        module_id: u64,
        title: String,
        description: String,
        video_url: String,
        article: String,
        order: u64,
        with_ai: bool,
        created_at: u64,
    ) -> bool {
        // only mentors can create lessons
        let account_id: AccountId = env::signer_account_id();

        // check if the module exists
        let module = self.get_module_by_id(module_id);
        if module.is_none() {
            log_str("Module does not exist");
            return false;
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

        return true;
    }

    pub fn update_lesson_details(
        &mut self,
        lesson_id: u64,
        title: String,
        description: String,
        updated_at: u64,
    ) -> bool {
        // Only the mentor of the lesson can update the lesson details
        let account_id: AccountId = env::signer_account_id();

        // check if the lesson exists
        let lesson: Option<Lesson> = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            return false;
        }

        // check if the mentor is the mentor of the lesson
        let lesson: Lesson = lesson.unwrap();
        let module: Module = self.get_module_by_id(lesson.module_id).unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the lesson can update the lesson details");
            return false;
        }

        // update the lesson details
        let mut lesson: Lesson = lesson.clone();
        lesson.title = title;
        lesson.description = description;
        lesson.updated_at = updated_at;

        // get the index of the lesson in the lessons vector
        let index: usize = self.lessons.iter().position(|l| l.id == lesson.id).unwrap();

        // update the lesson
        self.lessons.replace(index as u32, lesson.clone());

        // Log the update of the lesson details
        log_str(&format!("Lesson details updated: {}", lesson.title));

        true
    }

    pub fn delete_lesson(&mut self, lesson_id: u64) {
        // Only the mentor of the lesson can delete the lesson
        let account_id: AccountId = env::signer_account_id();

        // check if the lesson exists
        let lesson: Option<Lesson> = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            env::panic_str("Lesson does not exist")
        }

        // check if the mentor is the mentor of the lesson
        let lesson: Lesson = lesson.unwrap();
        let mut module: Module = self.get_module_by_id(lesson.module_id).unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the lesson can delete the lesson");
            env::panic_str("Only the mentor of the lesson can delete the lesson")
        }

        // remove the lesson from module lessons
        let index = module
            .lessons_ids
            .iter()
            .position(|l| l == &lesson_id)
            .unwrap();
        module.lessons_ids.remove(index);

        let lesson_order = lesson.order;

        // update lessons order that have order higher than the lesson order  in the module
        for lesson_id in module.lessons_ids.iter() {
            let lesson = self.get_lesson_by_id(*lesson_id).unwrap();
            if lesson.order > lesson_order {
                let mut lesson = lesson.clone();
                lesson.order -= 1;

                // get the index of the lesson in the lessons vector
                let index: usize = self.lessons.iter().position(|l| l.id == lesson.id).unwrap();

                // update the lesson
                self.lessons.replace(index as u32, lesson.clone());
            }
        }

        // get the index of the module in the modules vector
        let mod_index = self.modules.iter().position(|m| m.id == module.id).unwrap();

        // update the module
        self.modules.replace(mod_index as u32, module.clone());

        // remove lesson from the lessons vector
        let index = self.lessons.iter().position(|l| l.id == lesson_id).unwrap();

        let _removed_lesson = self.lessons.swap_remove(index as u32);

        // Log the deletion of the lesson
        log_str(&format!("Lesson deleted: {}", lesson.title));
    }

    pub fn publish_course(&mut self, course_id: u64, published_at: u64) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
        }

        // only the course mentor and admin can publish the course
        if !self.is_course_mentor(course_id, account_id.clone())
            && !self.is_admin(account_id.clone())
        {
            log_str("Only the course mentor or admin can publish the course");
            return false;
        }

        // change the course status to published
        let mut course = course.unwrap();
        course.status = "published".to_string();
        course.updated_at = published_at;

        // get the index of the course in the courses vector
        let index = self.courses.iter().position(|c| c.id == course.id).unwrap();

        // update the course
        self.courses.replace(index as u32, course.clone());

        return true;
    }

    pub fn archive_course(&mut self, course_id: u64) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
        }

        // only the course mentor and admin can archive the course
        if !self.is_course_mentor(course_id, account_id.clone())
            && !self.is_admin(account_id.clone())
        {
            log_str("Only the course mentor or admin can archive the course");
            return false;
        }

        // change the course status to archived
        let mut course = course.unwrap();
        course.status = "archived".to_string();

        // get the index of the course in the courses vector
        let index = self.courses.iter().position(|c| c.id == course.id).unwrap();

        // update the course
        self.courses.replace(index as u32, course.clone());

        return true;
    }

    pub fn save_course_to_cart(&mut self, course_id: u64, carted_at: u64) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
        }

        // check if the student exists
        let student = self.get_user_by_id(account_id.clone());
        if student.is_none() {
            log_str("Student does not exist");
            return false;
        }

        // check if the user is the mentor of that course
        if self.is_course_mentor(course_id, account_id.clone()) {
            log_str("User cannot cart their own course");
            return false;
        }

        // check if the course is already enrolled ny user
        if self.is_student_course_enrolled(course_id, account_id.clone()) {
            log_str("Student is already enrolled in the course");
            return false;
        }

        // check if the course is already carted by user
        if self.is_student_course_carted(course_id, account_id.clone()) {
            log_str("Course is already carted");
            return false;
        }

        // check if the course is already completed by user
        if self.is_student_course_completed(course_id, account_id.clone()) {
            log_str("Course is already completed");
            return false;
        }

        log_str(&format!("Saving Course to Cart..."));

        // create new enrollment with status carted
        let new_enrollment: Enrollment = Enrollment {
            id: self.enrollment_count,
            course_id,
            student_id: account_id.clone(),
            status: "carted".to_string(),
            progress: 0,
            carted_at,
            enrolled_at: None,
            updated_at: carted_at,
            course_review: None,
            completed_at: None,
        };

        // add enrollment to the enrollments vector
        self.enrollments.push(new_enrollment);

        // Increment the enrollment count
        self.enrollment_count += 1;

        // Log the carting of the course by the student
        log_str(&format!(
            "Course carted by student: {}",
            course.unwrap().title
        ));

        true
    }

    pub fn remove_course_from_cart(&mut self, course_id: u64) -> bool {
        let account_id: AccountId = env::signer_account_id();

        // check if the course exists
        let course = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
        }

        // check if the student exists
        let student = self.get_user_by_id(account_id.clone());
        if student.is_none() {
            log_str("Student does not exist");
            return false;
        }

        // check if the course is already carted by user
        if !self.is_student_course_carted(course_id, account_id.clone()) {
            log_str("Course is not carted");
            return false;
        }

        log_str(&format!("Removing Course from Cart..."));

        // get the enrollment of the course by the student
        let index = self
            .enrollments
            .iter()
            .position(|e| e.course_id == course_id && e.student_id == account_id)
            .unwrap();

        // remove the enrollment from the enrollments vector
        let _removed_enrollment = self.enrollments.swap_remove(index as u32);

        // Log the removal of the course from the cart
        log_str(&format!(
            "Course '{}' removed from cart by : {}",
            course.unwrap().title,
            account_id
        ));

        true
    }

    #[payable]
    pub fn enroll_all_carted_courses(&mut self, enrolled_at: u64) {
        let account_id: AccountId = env::signer_account_id();
        // get attached deposit
        let attached_deposit = env::attached_deposit();

        // get all the carted courses by the student
        let carted_courses: Vec<Course> = self.get_user_carted_courses(account_id.clone());

        // get the total price of the carted courses
        let total_courses_price: u128 =
            self.calculate_total_courses_price_with_fee(carted_courses.clone());

        log!(format!("Total courses price: {}", total_courses_price));
        log!(format!("Attached deposit: {}", attached_deposit));

        // check if the attached deposit is equal to the total courses price
        if attached_deposit < total_courses_price {
            log_str("Attached deposit is not equal to the total courses price");
            panic!("Attached deposit is not equal to the total courses price");
        }

        log_str(&format!("Enrolling Student in All Carted Courses..."));
        // getting all carted courses enrollments
        let user_carted_enrollment: Vec<Enrollment> =
            self.get_user_carted_enrollments(account_id.clone());

        for carted_enrollment in user_carted_enrollment.iter() {
            // update the enrollment status to enrolled
            let mut enrollment: Enrollment = carted_enrollment.clone();

            enrollment.status = "enrolled".to_string();
            enrollment.enrolled_at = Some(enrolled_at);
            enrollment.updated_at = enrolled_at;

            // get enrollment index
            let index = self
                .enrollments
                .iter()
                .position(|e| e.id == enrollment.id)
                .unwrap();

            // update the enrollment
            self.enrollments.replace(index as u32, enrollment.clone());

            log!(format!(
                "Update Enrollment status to enrolled: {:?}",
                enrollment.clone()
            ));

            // create new module progress for the student
            let course = self.get_course_by_id(enrollment.course_id).unwrap();
            for module_id in course.modules_ids.iter() {
                let module = self.get_module_by_id(*module_id).unwrap();
                let new_module_progress: ModuleProgress = ModuleProgress {
                    id: self.module_progress_count,
                    module_id: module.id,
                    student_id: account_id.clone(),
                    status: "not_started".to_string(),
                    is_enrolled: true,
                    progress: 0,
                    completed_at: None,
                };

                // add module progress to the module progresses vector
                self.module_progresses.push(new_module_progress);

                // increment the module progress count
                self.module_progress_count += 1;

                // enroll the student in the module quizz if it exists
                if module.quizz_id.is_some() {
                    let quizz = self.get_quizz_by_id(module.quizz_id.unwrap()).unwrap();
                    let new_quizz_progress: QuizzProgress = QuizzProgress {
                        id: self.quizz_count,
                        quizz_id: quizz.id,
                        student_id: account_id.clone(),
                        status: "not_started".to_string(),
                        is_enrolled: true,
                        try_count: 0,
                        is_submitted: false,
                        is_correct: false,
                        completed_at: None,
                    };

                    // add quizz progress to the quizz progresses vector
                    self.quizz_progresses.push(new_quizz_progress);

                    // increment the quizz progress count
                    self.quizz_progress_count += 1;
                }

                // enroll the student in the module lessons
                for lesson_id in module.lessons_ids.iter() {
                    let lesson = self.get_lesson_by_id(*lesson_id); 
                    if lesson.is_none() {
                        log_str(&format!(
                            "Lesson {} does not exist",
                            lesson_id
                        ));
                        continue;
                    }

                    // create new lesson progress for the student
                    let new_lesson_progress: LessonProgress = LessonProgress {
                        id: self.lesson_progress_count,
                        lesson_id: lesson_id.clone(),
                        student_id: account_id.clone(),
                        status: "not_started".to_string(),
                        is_enrolled: true,
                        completed_at: None,
                    };

                    // add lesson progress to the lesson progresses vector
                    self.lesson_progresses.push(new_lesson_progress);

                    // increment the lesson progress count
                    self.lesson_progress_count += 1;
                }
            }

            // Log the enrollment of the student in the course
            log_str(&format!(
                "Student enrolled in course: {}",
                course.title.clone()
            ));

            // trasnfer the course price to the course mentor
            let course_mentor = course.mentor_id;
            let transfer_amount = course.price;

            log!(format!(
                "Transfering {} NEAR to mentor: {}",
                course.price, course_mentor
            ));

            Promise::new(course_mentor).transfer(transfer_amount);
        }
    }

    pub fn submit_quizz(&mut self, quizz_id: u64, submitted_questions: Vec<Question>) {
        // only students can submit quizz
        let account_id: AccountId = env::signer_account_id();

        // check if the quizz exists
        let quizz = self.get_quizz_by_id(quizz_id);
        if quizz.is_none() {
            log_str("Quizz does not exist");
            env::panic_str("Quizz does not exist")
        }

        // get the quizz progress of the student
        let quizz_progress = self
            .get_quizz_progress(quizz_id, account_id.clone())
            .unwrap();

        // check if quizz is enrolled
        if !quizz_progress.is_enrolled {
            log_str("Quizz is not enrolled");
            env::panic_str("Quizz is not enrolled")
        }

        // check if quizz is already submitted correctly
        if quizz_progress.is_correct {
            log_str("Quizz is already complted correctly");
            env::panic_str("Quizz is already complted correctly")
        }

        // check if quizz questions === submitted questions
        let quizz = quizz.unwrap();
        let correct_questions = quizz.questions.clone();
        let mut correct_quizz = true;
        for (i, question) in correct_questions.iter().enumerate() {
            if question != &submitted_questions[i] {
                correct_quizz = false;
                break;
            }
        }

        // update the quizz progress based on correct_quizz value
        let mut quizz_progress = quizz_progress.clone();
        quizz_progress.is_submitted = true;
        quizz_progress.try_count += 1;
        quizz_progress.is_correct = correct_quizz.clone();
        if correct_quizz {
            // update the quizz progress status to completed
            quizz_progress.status = "completed".to_string();

            // get the index of the quizz progress in the quizz progresses vector
            let index = self
                .quizz_progresses
                .iter()
                .position(|qp| qp.id == quizz_progress.id)
                .unwrap();

            // update the quizz progress
            self.quizz_progresses
                .replace(index as u32, quizz_progress.clone());

            // get module progress
            let module_progress = self
                .get_module_progress(quizz.module_id, account_id.clone())
                .unwrap();
            // check if all lessons in the module are completed
            let module = self.get_module_by_id(quizz.module_id).unwrap();
            let lessons = module.lessons_ids;
            let mut all_lessons_completed = true;
            for lesson_id in lessons.iter() {
                let lesson_progress = self
                    .get_lesson_progress(*lesson_id, account_id.clone())
                    .unwrap();
                if lesson_progress.status != "completed" {
                    all_lessons_completed = false;
                    break;
                }
            }

            if all_lessons_completed {
                let mut module_progress = module_progress.clone();
                module_progress.status = "completed".to_string();
                module_progress.completed_at = Some(env::block_timestamp());

                // get the index of the module progress in the module progresses vector
                let mod_index = self
                    .module_progresses
                    .iter()
                    .position(|mp| mp.id == module_progress.id)
                    .unwrap();

                // update the module progress
                self.module_progresses
                    .replace(mod_index as u32, module_progress.clone());

                // shwo msg
                log_str(&format!("Module completed: {}", module.title));
            }

            // check if all modules in the course are completed
            let course_id = module.course_id;
            let course = self.get_course_by_id(course_id).unwrap();
            let modules = course.clone().modules_ids;
            let mut all_modules_completed = true;
            for module_id in modules.iter() {
                let module_progress = self
                    .get_module_progress(*module_id, account_id.clone())
                    .unwrap();
                if module_progress.status != "completed" {
                    all_modules_completed = false;
                    break;
                }
            }

            // update the enrollment staus to completed if all modules are completed
            if all_modules_completed {
                let course_id = module.course_id;
                let enrollment = self.get_enrollment(course_id, account_id.clone()).unwrap();
                let mut enrollment = enrollment.clone();
                enrollment.status = "completed".to_string();
                enrollment.completed_at = Some(env::block_timestamp());

                // get the index of the enrollment in the enrollments vector
                let en_index = self
                    .enrollments
                    .iter()
                    .position(|e| e.id == enrollment.id)
                    .unwrap();

                // update the enrollment
                self.enrollments
                    .replace(en_index as u32, enrollment.clone());

                // Log the completion of the course
                log_str(&format!("Course completed: {}", course.title));
            }
        } else {
            // update the quizz progress status to submitted
            quizz_progress.status = "submitted".to_string();

            // get the index of the quizz progress in the quizz progresses vector
            let index = self
                .quizz_progresses
                .iter()
                .position(|qp| qp.id == quizz_progress.id)
                .unwrap();

            // update the quizz progress
            self.quizz_progresses
                .replace(index as u32, quizz_progress.clone());
        }

        // Log the submission of the quizz
        log_str(&format!("Quizz submitted: {}", quizz.title));
    }

    pub fn complete_lesson(&mut self, lesson_id: u64) -> bool {
        // only students can complete lessons
        let account_id: AccountId = env::signer_account_id();

        // check if the lesson exists
        let lesson = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            return false;
        }

        // check if the student is enrolled in the lesson course
        if !self.is_student_lesson_enrolled(lesson_id, account_id.clone()) {
            log_str("Student is not enrolled in the lesson course");
            return false;
        }

        // check if the student has already completed the lesson
        if self.is_student_lesson_completed(lesson_id, account_id.clone()) {
            log_str("Student has already completed the lesson");
            return false;
        }

        log_str(&format!("Completing Lesson..."));
        let module_id = lesson.clone().unwrap().module_id;

        // update the lesson progress
        let lesson_progress = self
            .get_lesson_progress(lesson_id, account_id.clone())
            .unwrap();
        let mut lesson_progress = lesson_progress.clone();
        lesson_progress.status = "completed".to_string();
        lesson_progress.completed_at = Some(env::block_timestamp());

        //  get index of the lesson progress in the lesson progresses vector
        let less_index = self
            .lesson_progresses
            .iter()
            .position(|lp| lp.id == lesson_progress.id)
            .unwrap();

        // update the lesson progress
        self.lesson_progresses
            .replace(less_index as u32, lesson_progress.clone());

        // update teh module progress status to started
        let module_progress = self
            .get_module_progress(module_id, account_id.clone())
            .unwrap();

        if module_progress.status == "not_started" {
            let mut module_progress = module_progress.clone();
            module_progress.status = "started".to_string();

            // get the index of the module progress in the module progresses vector
            let mod_index = self
                .module_progresses
                .iter()
                .position(|mp| mp.id == module_progress.id)
                .unwrap();

            // update the module progress
            self.module_progresses
                .replace(mod_index as u32, module_progress.clone());
        }

        // check if all the lessons in the module are completed
        let module = self.get_module_by_id(module_id).unwrap();
        // check if module quizz status = complted if quizz existed
        let mut quizz_completed = true;
        if module.quizz_id.is_some() {
            let quizz_id = module.quizz_id.unwrap();
            let quizz_progress = self
                .get_quizz_progress(quizz_id, account_id.clone())
                .unwrap();
            if quizz_progress.status != "completed" {
                quizz_completed = false;
            }
        }
        let lessons = module.lessons_ids;
        let mut all_lessons_completed = true;
        for lesson_id in lessons.iter() {
            let lesson_progress = self
                .get_lesson_progress(*lesson_id, account_id.clone())
                .unwrap();
            if lesson_progress.status != "completed" {
                all_lessons_completed = false;
                break;
            }
        }

        // update the module progress status to completed
        if all_lessons_completed && quizz_completed {
            let mut module_progress = module_progress.clone();
            module_progress.status = "completed".to_string();
            module_progress.completed_at = Some(env::block_timestamp());

            // get the index of the module progress in the module progresses vector
            let mod_index = self
                .module_progresses
                .iter()
                .position(|mp| mp.id == module_progress.id)
                .unwrap();

            // update the module progress
            self.module_progresses
                .replace(mod_index as u32, module_progress.clone());

            // shwo msg
            log_str(&format!("Module completed: {}", module.title));
        }

        // check if all the modules in the course are completed
        let course_id = module.course_id;
        let course = self.get_course_by_id(course_id).unwrap();
        let modules = course.clone().modules_ids;
        let mut all_modules_completed = true;
        for module_id in modules.iter() {
            let module_progress = self
                .get_module_progress(*module_id, account_id.clone())
                .unwrap();
            if module_progress.status != "completed" {
                all_modules_completed = false;
                break;
            }
        }

        // Log the completion of the lesson
        log_str(&format!("Lesson completed: {}", lesson.unwrap().title));

        // update the enrollment staus to completed if all modules are completed
        if all_modules_completed {
            let course_id = module.course_id;
            let enrollment = self.get_enrollment(course_id, account_id.clone()).unwrap();
            let mut enrollment = enrollment.clone();
            enrollment.status = "completed".to_string();
            enrollment.completed_at = Some(env::block_timestamp());

            // get the index of the enrollment in the enrollments vector
            let en_index = self
                .enrollments
                .iter()
                .position(|e| e.id == enrollment.id)
                .unwrap();

            // update the enrollment
            self.enrollments
                .replace(en_index as u32, enrollment.clone());

            // Log the completion of the course
            log_str(&format!("Course completed: {}", course.title));
        }

        true
    }

    pub fn add_video_to_lesson(&mut self, lesson_id: u64, ipfs_url: String) -> bool {
        // Only the mentor of the lesson can add video to the lesson
        let account_id: AccountId = env::signer_account_id();

        // check if the lesson exists
        let lesson: Option<Lesson> = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            return false;
        }

        // check if the mentor is the mentor of the lesson
        let lesson: Lesson = lesson.unwrap();
        let module: Module = self.get_module_by_id(lesson.module_id).unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the lesson can add video to the lesson");
            return false;
        }

        // update the lesson video url
        let mut lesson: Lesson = lesson.clone();
        lesson.video_url = ipfs_url;

        // get the index of the lesson in the lessons vector
        let index: usize = self.lessons.iter().position(|l| l.id == lesson.id).unwrap();

        // update the lesson
        self.lessons.replace(index as u32, lesson.clone());

        // Log the addition of the video to the lesson
        log_str(&format!(
            "Video '{}' added to lesson: {}",
            lesson.video_url, lesson.title
        ));

        true
    }

    pub fn add_article_to_lesson(&mut self, lesson_id: u64, article: String) -> bool {
        // Only the mentor of the lesson can add article to the lesson
        let account_id: AccountId = env::signer_account_id();

        // check if the lesson exists
        let lesson: Option<Lesson> = self.get_lesson_by_id(lesson_id);
        if lesson.is_none() {
            log_str("Lesson does not exist");
            return false;
        }

        // check if the mentor is the mentor of the lesson
        let lesson: Lesson = lesson.unwrap();
        let module: Module = self.get_module_by_id(lesson.module_id).unwrap();
        let course: Course = self.get_course_by_id(module.course_id).unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the lesson can add article to the lesson");
            return false;
        }

        // update the lesson article
        let mut lesson: Lesson = lesson.clone();
        lesson.article = article;

        // get the index of the lesson in the lessons vector
        let index: usize = self.lessons.iter().position(|l| l.id == lesson.id).unwrap();

        // update the lesson
        self.lessons.replace(index as u32, lesson.clone());

        // Log the addition of the article to the lesson
        log_str(&format!("Article added to lesson: {}", lesson.title));

        true
    }

    pub fn update_course_details(
        &mut self,
        course_id: u64,
        title: String,
        description: String,
        price: U128,
        category: String,
        updated_at: u64,
    ) -> bool {
        // Only the mentor of the course can update the course details
        let account_id: AccountId = env::signer_account_id();

        // check if the course exists
        let course: Option<Course> = self.get_course_by_id(course_id);
        if course.is_none() {
            log_str("Course does not exist");
            return false;
        }

        // check if the mentor is the mentor of the course
        let course: Course = course.unwrap();
        if course.mentor_id != account_id {
            log_str("Only the mentor of the course can update the course details");
            return false;
        }

        // update the course details
        let mut course: Course = course.clone();
        course.title = title;
        course.description = description;
        course.price = u128::from(price);
        course.category = category;
        course.updated_at = updated_at;

        // get the index of the course in the courses vector
        let index: usize = self.courses.iter().position(|c| c.id == course.id).unwrap();

        // update the course
        self.courses.replace(index as u32, course.clone());

        // Log the update of the course details
        log_str(&format!("Course details updated: {}", course.title));

        true
    }
}
