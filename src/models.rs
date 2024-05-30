use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub account_id: AccountId,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub by_google: bool,
    pub certifications: Option<Vec<Certification>>,
    pub picture: String,
    pub created_at: u64,
    pub updated_at: u64,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Certification {
    pub id: u64,
    pub title: String,
    pub from: String,
    pub to: String,
    pub picture: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Course {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub level: String,
    pub duration: String,
    pub status: String,
    pub requirements: Vec<String>,
    pub objectives: Vec<String>,
    pub category: String,

    pub picture: String,
    pub with_ai: bool,
    pub price: u128,
    pub mentor_id: AccountId,
    pub modules_ids: Vec<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Module {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub order: u64,
    pub with_ai: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub course_id: u64,
    pub lessons_ids: Vec<u64>,
    pub quizz_id: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Lesson {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub article: String,
    pub order: u64,
    pub with_ai: bool,
    pub module_id: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Quizz {
    pub id: u64,
    pub module_id: u64,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub with_ai: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
    pub text: String,
    pub answers: Vec<Answer>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    pub text: String,
    pub is_correct: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullLesson {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub article: String,
    pub order: u64,
    pub with_ai: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullModule {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub order: u64,
    pub with_ai: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub lessons: Vec<FullLesson>,
    pub quizz: Option<Quizz>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullCourse {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub level: String,
    pub duration: String,
    pub status: String,
    pub requirements: Vec<String>,
    pub objectives: Vec<String>,
    pub category: String,
    pub picture: String,
    pub with_ai: bool,
    pub price: u128,
    pub mentor: User,
    pub modules: Vec<FullModule>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Enrollment {
    pub id: u64,
    pub course_id: u64,
    pub student_id: AccountId,
    pub status: String,
    pub progress: u16,
    pub carted_at: u64,
    pub enrolled_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub course_review: Option<u64>,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ModuleProgress {
    pub id: u64,
    pub module_id: u64,
    pub student_id: AccountId,
    pub status: String,
    pub is_enrolled: bool,
    pub progress: u16,
    pub completed_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LessonProgress {
    pub id: u64,
    pub lesson_id: u64,
    pub student_id: AccountId,
    pub status: String,
    pub is_enrolled: bool,
    pub completed_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct QuizzProgress {
    pub id: u64,
    pub quizz_id: u64,
    pub student_id: AccountId,
    pub status: String,
    pub try_count: u16,
    pub is_enrolled: bool,
    pub is_submitted: bool,
    pub is_correct: bool,
    pub completed_at: Option<u64>,

}



#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullEnrollment {
    pub id: u64,
    pub course: Course,
    pub student: User,
    pub modules: Vec<FullModuleProgress>,
    pub status: String,
    pub progress: u16,
    pub carted_at: u64,
    pub enrolled_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub course_review: Option<u64>,
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullModuleProgress {
    pub id: u64,
    pub module: FullModule,
    pub student: User,
    pub lessons: Vec<FullLessonProgress>,
    pub status: String,
    pub is_enrolled: bool,
    pub progress: u16,
    pub quizz: Option<FullQuizzProgress>,
    pub completed_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullQuizzProgress {
    pub id: u64,
    pub quizz: Quizz,
    pub student: User,
    pub status: String,
    pub try_count: u16,
    pub is_enrolled: bool,
    pub is_submitted: bool,
    pub is_correct: bool,
    pub completed_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FullLessonProgress {
    pub id: u64,
    pub lesson: FullLesson,
    pub student: User,
    pub status: String,
    pub is_enrolled: bool,
    pub completed_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseWithProgress {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub level: String,
    pub duration: String,
    pub status: String,
    pub requirements: Vec<String>,
    pub objectives: Vec<String>,
    pub category: String,

    pub picture: String,
    pub with_ai: bool,
    pub price: u128,
    pub mentor_id: AccountId,
    pub modules_ids: Vec<u64>,
    pub created_at: u64,
    pub updated_at: u64,
    pub progress: u64,
}
