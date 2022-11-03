trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
struct Me();

impl Person for Me {
    fn name(&self) -> String {
      "Dan".to_string()
    }
}

impl Student for Me {
    fn university(&self) -> String {
      "Internet University".to_string()
    }
}

impl Programmer for Me {
    fn fav_language(&self) -> String {
      "Rust of course!".to_string()
    }
}

impl CompSciStudent for Me{
    fn git_username(&self) -> String {
      "DanWillans".to_string()
    }
}

pub fn supertraits() {
  let me = Me();
  println!("{}", comp_sci_student_greeting(&me));
}