import { Worker, NearAccount, NEAR, ONE_NEAR } from "near-workspaces";
import { parseNEAR, toYocto } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const ayoub = await root.createSubAccount("ayoub");
  const ahmed = await root.createSubAccount("ahmed");
  const contract = await root.createSubAccount("test-account");

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(process.argv[2]);

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, ayoub, ahmed };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("mentor create full course & student enrolled it after paying course price to mentor", async (t) => {
  const { contract, ayoub, ahmed } = t.context.accounts;

  // craete mentor
  const mentorSucess = await ayoub.call(contract, "create_user", {
    name: "Ayoub",
    username: "ayoub",
    role: "mentor",
    email: "ayoub@gmail.com",
    password: "123",
    by_google: false,
    bio: "I am a software engineer",
    skills: ["JavaScript", "TypeScript", "React", "Node.js"],
    certifications: ["AWS", "Google Cloud", "Docker"],
    education: ["Bachelor's degree in Computer Science"],
    picture: "https://avatars.githubusercontent.com/u/47231147?v=4",
    created_at: new Date().getTime(),
  });

  t.is(mentorSucess, true);

  const mentor = await contract.view("get_user_by_id", { id: ayoub.accountId });

  console.log(mentor);

  // mentor create course
  const courseSucess = await ayoub.call(contract, "create_course", {
    title: "React From Scratch",
    description: "React course",
    level: "beginner",
    duration: "1 month",
    category: "web development",
    with_ai: false,
    tags: ["React", "JavaScript", "Frontend"],
    price: 6,
    picture: "https://avatars.githubusercontent.com/u/47231147?v=4",
    created_at: new Date().getTime(),
  });

  const course = await contract.view("get_course_by_id", { course_id: 0 });

  console.log("Course : ", course);

  t.is(courseSucess, true);

  // mentor create a module for the course
  const moduleSucess = await ayoub.call(contract, "create_module", {
    course_id: 0,
    title: "React Basics",
    description: "React basics",
    status: "created",
    order: 1,
    with_ai: false,
    created_at: new Date().getTime(),
  });

  const module = await contract.view("get_module_by_id", { module_id: 0 });

  console.log("Module : ", module);

  t.is(moduleSucess, true);

  // mentor create a lesson for the module
  const lessonSucess = await ayoub.call(contract, "create_lesson", {
    module_id: 0,
    title: "React State",
    description: "React state",
    order: 1,
    video_url: "https://www.youtube.com/watch?v=Ke90Tje7VS0",
    article:
      "shhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhdjhssgfgsfqlslsfihfsdbfshsffsjfsfsbfldidvqsgsvlkfolgyvcvsdmlqùsmhfbgfjfjdfsmsfyfbtbu",
    with_ai: false,
    created_at: new Date().getTime(),
  });

  const lesson = await contract.view("get_lesson_by_id", { lesson_id: 0 });

  console.log("Lesson : ", lesson);

  t.is(lessonSucess, true);

  // get the full course
  const fullCourse = await contract.view("get_full_course", { course_id: 0 });

  console.log("Full Course : ", fullCourse);

  // publish the course
  const publishSucess = await ayoub.call(contract, "publish_course", {
    course_id: 0,
    published_at: new Date().getTime(),
  });

  t.is(publishSucess, true);

  // create student
  const studentSucess = await ahmed.call(contract, "create_user", {
    name: "Ahmed",
    username: "ahmed",
    role: "student",
    email: "ahmed@gmail.com",
    password: "123",
    by_google: false,
    bio: "I am a student",
    skills: ["JavaScript", "TypeScript", "React", "Node.js"],
    certifications: ["AWS", "Google Cloud", "Docker"],
    education: ["Bachelor's degree in Computer Science"],
    picture: "https://avatars.githubusercontent.com/u/47231147?v=4",
    created_at: new Date().getTime(),
  });

  t.is(studentSucess, true);

  // student enroll in the course
  const enrollSucess = await ahmed.call(
    contract,
    "enroll_course",
    {
      course_id: 0,
      enrolled_at: new Date().getTime(),
    },
    {
      attachedDeposit: toYocto("7"),
    }
  );

  t.is(enrollSucess, true);

  // get student enrolled courses
  const studentCourses = await contract.view("get_student_courses", {
    student_id: ahmed.accountId,
  });

  console.log("Student Courses : ", studentCourses);
});
