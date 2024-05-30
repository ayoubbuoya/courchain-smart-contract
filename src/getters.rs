use crate::models::*;
use crate::{Contract, ContractExt};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    pub fn get_quizzes(&self) -> Vec<Quizz> {
        let mut quizz_list: Vec<Quizz> = vec![];
        for quizz in self.quizzes.iter() {
            quizz_list.push(quizz.clone());
        }
        quizz_list
    }

    pub fn get_users(&self) -> Vec<User> {
        let mut user_list: Vec<User> = vec![];
        for user in self.users.iter() {
            user_list.push(user.clone()); // Clone each User struct
        }
        user_list
    }

    pub fn get_courses(&self) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for course in self.courses.iter() {
            course_list.push(course.clone());
        }
        course_list
    }

    pub fn get_full_courses(&self) -> Vec<FullCourse> {
        let mut full_courses_list: Vec<FullCourse> = vec![];
        for course in self.courses.iter() {
            let full_course = self.get_full_course(course.id);
            full_courses_list.push(full_course.unwrap().clone())
        }
        full_courses_list
    }

    pub fn get_modules(&self) -> Vec<Module> {
        let mut module_list: Vec<Module> = vec![];
        for module in self.modules.iter() {
            module_list.push(module.clone());
        }
        module_list
    }

    pub fn get_lessons(&self) -> Vec<Lesson> {
        let mut lesson_list: Vec<Lesson> = vec![];
        for lesson in self.lessons.iter() {
            lesson_list.push(lesson.clone());
        }
        lesson_list
    }

    pub fn get_user_by_id(&self, id: AccountId) -> Option<User> {
        for user in self.users.iter() {
            if user.account_id == id {
                return Some(user.clone());
            }
        }
        None
    }

    pub fn get_quizz_by_id(&self, quizz_id: u64) -> Option<Quizz> {
        for quizz in self.quizzes.iter() {
            if quizz.id == quizz_id {
                return Some(quizz.clone());
            }
        }
        None
    }

    pub fn get_user_by_username(&self, username: String) -> Option<User> {
        for user in self.users.iter() {
            if user.username == username {
                return Some(user.clone());
            }
        }
        None
    }

    pub fn get_user_by_email(&self, email: String) -> Option<User> {
        for user in self.users.iter() {
            if user.email == email {
                return Some(user.clone());
            }
        }
        None
    }

    pub fn get_course_by_id(&self, course_id: u64) -> Option<Course> {
        for course in self.courses.iter() {
            if course.id == course_id {
                return Some(course.clone());
            }
        }
        None
    }

    pub fn get_user_carted_courses(&self, account_id: AccountId) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.student_id == account_id && enrollment.status == "carted" {
                let course = self.get_course_by_id(enrollment.course_id);
                if let Some(course) = course {
                    course_list.push(course);
                }
            }
        }
        course_list
    }

    pub fn get_user_carted_enrollments(&self, account_id: AccountId) -> Vec<Enrollment> {
        let mut enrollment_list: Vec<Enrollment> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.student_id == account_id && enrollment.status == "carted" {
                enrollment_list.push(enrollment.clone());
            }
        }
        enrollment_list
    }

    pub fn get_course_modules(&self, course_id: u64) -> Vec<Module> {
        let mut module_list: Vec<Module> = vec![];
        let course = self.get_course_by_id(course_id);
        if let Some(course) = course {
            for module_id in course.modules_ids.iter() {
                let module = self.get_module_by_id(*module_id);
                if let Some(module) = module {
                    module_list.push(module);
                }
            }
        }
        module_list
    }

    pub fn get_course_students(&self, course_id: u64) -> Vec<User> {
        let mut student_list: Vec<User> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.course_id == course_id
                && (enrollment.status == "enrolled" || enrollment.status == "completed")
            {
                let student = self.get_user_by_id(enrollment.student_id.clone());
                if let Some(student) = student {
                    student_list.push(student);
                }
            }
        }
        student_list
    }

    pub fn get_module_by_id(&self, module_id: u64) -> Option<Module> {
        for module in self.modules.iter() {
            if module.id == module_id {
                return Some(module.clone());
            }
        }
        None
    }

    pub fn get_module_lessons(&self, module_id: u64) -> Vec<Lesson> {
        let mut lesson_list: Vec<Lesson> = vec![];
        for lesson in self.lessons.iter() {
            if lesson.module_id == module_id {
                lesson_list.push(lesson.clone());
            }
        }
        lesson_list
    }

    pub fn get_published_courses(&self) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for course in self.courses.iter() {
            if course.status == "published" {
                course_list.push(course.clone());
            }
        }
        course_list
    }

    pub fn get_archived_courses(&self) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for course in self.courses.iter() {
            if course.status == "archived" {
                course_list.push(course.clone());
            }
        }
        course_list
    }

    pub fn get_enrollment(&self, course_id: u64, account_id: AccountId) -> Option<Enrollment> {
        for enrollment in self.enrollments.iter() {
            if enrollment.course_id == course_id && enrollment.student_id == account_id {
                return Some(enrollment.clone());
            }
        }
        None
    }

    pub fn get_mentor_created_courses(&self, mentor_id: AccountId) -> Vec<CourseWithProgress> {
        let mut course_list: Vec<CourseWithProgress> = vec![];
        for course in self.courses.iter() {
            if course.mentor_id == mentor_id {
                let progress = 0;
                let course_with_progress = CourseWithProgress {
                    id: course.id,
                    title: course.title.clone(),
                    description: course.description.clone(),
                    status: course.status.clone(),
                    created_at: course.created_at,
                    updated_at: course.updated_at,
                    mentor_id: course.mentor_id.clone(),
                    level: course.level.clone(),
                    duration: course.duration.clone(),
                    requirements: course.requirements.clone(),
                    objectives: course.objectives.clone(),
                    category: course.category.clone(),
                    picture: course.picture.clone(),
                    with_ai: course.with_ai,
                    price: course.price,
                    modules_ids: course.modules_ids.clone(),
                    progress,
                };
                course_list.push(course_with_progress);
            }
        }
        course_list
    }

    pub fn get_student_courses(&self, student_id: AccountId) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.student_id == student_id {
                let course = self.get_course_by_id(enrollment.course_id);
                if let Some(course) = course {
                    course_list.push(course);
                }
            }
        }
        course_list
    }

    pub fn get_lesson_by_id(&self, lesson_id: u64) -> Option<Lesson> {
        for lesson in self.lessons.iter() {
            if lesson.id == lesson_id {
                return Some(lesson.clone());
            }
        }
        None
    }

    pub fn get_module_quizz(&self, module: Module) -> Option<Quizz> {
        if let Some(quizz_id) = module.quizz_id {
            for quizz in self.quizzes.iter() {
                if quizz.id == quizz_id {
                    return Some(quizz.clone());
                }
            }
        }
        None
    }
    pub fn get_full_course(&self, course_id: u64) -> Option<FullCourse> {
        let course = self.get_course_by_id(course_id);
        if let Some(course) = course {
            let mut modules: Vec<FullModule> = vec![];

            let mentor = self.get_user_by_id(course.mentor_id.clone());

            for module_id in course.modules_ids.iter() {
                let module = self.get_module_by_id(*module_id);
                if let Some(module) = module {
                    let mut lessons: Vec<FullLesson> = vec![];
                    let quizz = self.get_module_quizz(module.clone());
                    for lesson_id in module.lessons_ids.iter() {
                        let lesson = self.get_lesson_by_id(*lesson_id);
                        if let Some(lesson) = lesson {
                            lessons.push(FullLesson {
                                id: lesson.id,
                                title: lesson.title,
                                description: lesson.description,
                                video_url: lesson.video_url,
                                article: lesson.article,
                                order: lesson.order,
                                with_ai: lesson.with_ai,
                                created_at: lesson.created_at,
                                updated_at: lesson.updated_at,
                            });
                        }
                    }
                    modules.push(FullModule {
                        id: module.id,
                        title: module.title,
                        description: module.description,
                        status: module.status,
                        order: module.order,
                        with_ai: module.with_ai,
                        created_at: module.created_at,
                        updated_at: module.updated_at,
                        lessons,
                        quizz,
                    });
                }
            }

            return Some(FullCourse {
                id: course.id,
                title: course.title.clone(),
                description: course.description.clone(),
                status: course.status.clone(),
                created_at: course.created_at,
                updated_at: course.updated_at,
                modules,
                level: course.level.clone(),
                duration: course.duration.clone(),
                requirements: course.requirements.clone(),
                objectives: course.objectives.clone(),
                category: course.category.clone(),

                picture: course.picture.clone(),
                with_ai: course.with_ai,
                price: course.price,
                mentor: mentor.unwrap(),
            });
        }
        None
    }

    pub fn get_lesson_progress(
        &self,
        lesson_id: u64,
        student_id: AccountId,
    ) -> Option<LessonProgress> {
        for lesson_progress in self.lesson_progresses.iter() {
            if lesson_progress.lesson_id == lesson_id && lesson_progress.student_id == student_id {
                return Some(lesson_progress.clone());
            }
        }
        None
    }

    pub fn get_module_progress(
        &self,
        module_id: u64,
        student_id: AccountId,
    ) -> Option<ModuleProgress> {
        for module_progress in self.module_progresses.iter() {
            if module_progress.module_id == module_id && module_progress.student_id == student_id {
                return Some(module_progress.clone());
            }
        }
        None
    }

    pub fn get_user_carted_courses_prices_with_fee(&self, account_id: AccountId) -> u128 {
        let carted_coureses: Vec<Course> = self.get_user_carted_courses(account_id);
        let total_price: u128 = self.calculate_total_courses_price_with_fee(carted_coureses);

        total_price
    }

    pub fn get_user_enrollments(&self, account_id: AccountId) -> Vec<Enrollment> {
        let mut enrollment_list: Vec<Enrollment> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.student_id == account_id {
                enrollment_list.push(enrollment.clone());
            }
        }
        enrollment_list
    }

    pub fn get_user_enrolled_courses(&self, account_id: AccountId) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for enrollment in self.enrollments.iter() {
            if (enrollment.student_id == account_id)
                && (enrollment.status == "enrolled" || enrollment.status == "completed")
            {
                let course = self.get_course_by_id(enrollment.course_id);
                if let Some(course) = course {
                    course_list.push(course);
                }
            }
        }
        course_list
    }

    pub fn get_user_enrolled_full_courses(&self, account_id: AccountId) -> Vec<FullCourse> {
        let mut full_course_list: Vec<FullCourse> = vec![];
        let enrolled_courses = self.get_user_enrolled_courses(account_id);
        for course in enrolled_courses.iter() {
            let full_course = self.get_full_course(course.id);
            if let Some(full_course) = full_course {
                full_course_list.push(full_course);
            }
        }
        full_course_list
    }

    pub fn get_user_completed_courses(&self, account_id: AccountId) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for enrollment in self.enrollments.iter() {
            if enrollment.student_id == account_id && enrollment.status == "completed" {
                let course = self.get_course_by_id(enrollment.course_id);
                if let Some(course) = course {
                    course_list.push(course);
                }
            }
        }
        course_list
    }

    pub fn get_user_completed_full_courses(&self, account_id: AccountId) -> Vec<FullCourse> {
        let mut full_course_list: Vec<FullCourse> = vec![];
        let enrolled_courses = self.get_user_completed_courses(account_id);
        for course in enrolled_courses.iter() {
            let full_course = self.get_full_course(course.id);
            if let Some(full_course) = full_course {
                full_course_list.push(full_course);
            }
        }
        full_course_list
    }

    pub fn get_full_module_by_id(&self, module_id: u64) -> Option<FullModule> {
        let module = self.get_module_by_id(module_id);
        if let Some(module) = module {
            let mut lessons: Vec<FullLesson> = vec![];
            for lesson_id in module.lessons_ids.iter() {
                let lesson = self.get_lesson_by_id(*lesson_id);
                if let Some(lesson) = lesson {
                    lessons.push(FullLesson {
                        id: lesson.id,
                        title: lesson.title,
                        description: lesson.description,
                        video_url: lesson.video_url,
                        article: lesson.article,
                        order: lesson.order,
                        with_ai: lesson.with_ai,
                        created_at: lesson.created_at,
                        updated_at: lesson.updated_at,
                    });
                }
            }
            let quizz = self.get_module_quizz(module.clone());
            return Some(FullModule {
                id: module.id,
                title: module.title,
                description: module.description,
                status: module.status,
                order: module.order,
                with_ai: module.with_ai,
                created_at: module.created_at,
                updated_at: module.updated_at,
                lessons,
                quizz: quizz,
            });
        }

        None
    }

    pub fn get_full_quizz_progress(
        &self,
        quizz_id: u64,
        student_id: AccountId,
    ) -> Option<FullQuizzProgress> {
        for quizz_progress in self.quizz_progresses.iter() {
            if quizz_progress.quizz_id == quizz_id && quizz_progress.student_id == student_id {
                let quizz = self.get_quizz_by_id(quizz_id);
                if let Some(quizz) = quizz {
                    return Some(FullQuizzProgress {
                        id: quizz_progress.id,
                        quizz: quizz.clone(),
                        student: self.get_user_by_id(student_id).unwrap(),
                        status: quizz_progress.status.clone(),
                        is_enrolled: quizz_progress.is_enrolled,
                        try_count: quizz_progress.try_count,
                        is_submitted: quizz_progress.is_submitted,
                        is_correct: quizz_progress.is_correct,
                        completed_at: quizz_progress.completed_at,
                    });
                }
            }
        }
        None
    }

    pub fn get_student_enrolled_course(
        &self,
        course_id: u64,
        student_id: AccountId,
    ) -> Option<FullEnrollment> {
        for enrollment in self.enrollments.iter() {
            if enrollment.course_id == course_id && enrollment.student_id == student_id.clone() {
                let course = self.get_course_by_id(course_id);
                if let Some(course) = course {
                    let student = self.get_user_by_id(student_id.clone());
                    if let Some(student) = student {
                        let mut modules: Vec<FullModuleProgress> = vec![];
                        for module_id in course.modules_ids.iter() {
                            let module = self.get_full_module_by_id(*module_id);
                            if let Some(module) = module {
                                let module_progress =
                                    self.get_module_progress(module.id, student_id.clone());
                                if let Some(module_progress) = module_progress {
                                    let mut lessons: Vec<FullLessonProgress> = vec![];
                                    let quizz = module.clone().quizz;
                                    let mut quizz_progress = None;
                                    if let Some(quizz) = quizz {
                                        quizz_progress = self
                                            .get_full_quizz_progress(quizz.id, student_id.clone());
                                    }
                                    for lesson in module.lessons.iter() {
                                        let lesson_progress =
                                            self.get_lesson_progress(lesson.id, student_id.clone());
                                        if let Some(lesson_progress) = lesson_progress {
                                            lessons.push(FullLessonProgress {
                                                id: lesson.id,
                                                lesson: lesson.clone(),
                                                student: student.clone(),
                                                status: lesson_progress.status,
                                                is_enrolled: lesson_progress.is_enrolled,
                                                completed_at: lesson_progress.completed_at,
                                            });
                                        }
                                    }
                                    modules.push(FullModuleProgress {
                                        id: module_progress.id,
                                        module: module.clone(),
                                        student: student.clone(),
                                        lessons,
                                        quizz: quizz_progress,
                                        status: module_progress.status,
                                        is_enrolled: module_progress.is_enrolled,
                                        progress: module_progress.progress,
                                        completed_at: module_progress.completed_at,
                                    });
                                }
                            }
                        }

                        return Some(FullEnrollment {
                            id: enrollment.id,
                            course: course.clone(),
                            student: student.clone(),
                            modules,
                            status: enrollment.status.clone(),
                            progress: enrollment.progress,
                            carted_at: enrollment.carted_at,
                            enrolled_at: enrollment.enrolled_at,
                            completed_at: enrollment.completed_at,
                            course_review: enrollment.course_review,
                            updated_at: enrollment.updated_at,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn get_quizz_progress(
        &self,
        quizz_id: u64,
        student_id: AccountId,
    ) -> Option<QuizzProgress> {
        for quizz_progress in self.quizz_progresses.iter() {
            if quizz_progress.quizz_id == quizz_id && quizz_progress.student_id == student_id {
                return Some(quizz_progress.clone());
            }
        }
        None
    }

    pub fn get_user_created_courses(&self, account_id: AccountId) -> Vec<Course> {
        let mut course_list: Vec<Course> = vec![];
        for course in self.courses.iter() {
            if course.mentor_id == account_id {
                course_list.push(course.clone());
            }
        }
        course_list
    }

    pub fn get_user_created_full_courses(&self, account_id: AccountId) -> Vec<FullCourse> {
        let mut full_course_list: Vec<FullCourse> = vec![];
        let created_courses = self.get_user_created_courses(account_id);
        for course in created_courses.iter() {
            let full_course = self.get_full_course(course.id);
            if let Some(full_course) = full_course {
                full_course_list.push(full_course);
            }
        }
        full_course_list
    }
}
