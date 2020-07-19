use chrono::{DateTime, Datelike, Timelike, Utc};



#[derive(Debug)]
enum Priority {
    High,
    Medium,
    Low,
}

enum Status {
    Todo,
    InProgress,
    Done,
}

//Todo use a Crate for terminal colors..maybe colored?
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

//TODO: make an asynchronos function that saves/keep track of and run the Reminders that are collected from the saved Remindables
trait Remindable<'a> {
    fn set_reminder(date: DateTime<Utc>, message: &'a str) -> Reminder;
}

struct Reminder<'a> {
    duration: Duration,
    message: &'a str,
}
impl<'a> Reminder<'a> {
    fn new(reminder_date: DateTime<Utc>, message: &'a str) -> Reminder<'a> {
        let duration = Duration::new(Utc::now(), reminder_date);
        Reminder {
            duration: duration,
            message: message,
        }
    }
}

#[derive(Debug)]
struct Note<'a> {
    title: &'a str,
    body: &'a str,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl<'a> Remindable<'a> for Note<'a> {
    //another function(not created yet) will save and keep track of that returned Reminder
    //TODO: update the previous comment after making the handler function
    fn set_reminder(date: DateTime<Utc>, message: &'a str) -> Reminder<'a> {
        Reminder::new(date, message)
    }
}

impl<'a> Note<'a> {
    fn new(title: &'a str, body: &'a str) -> Note<'a> {
        Note {
            title: title,
            body: body,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn edit_title(&mut self, new_title: &'a str) {
        self.title = new_title;
        self.updated_at = Utc::now();
    }

    fn edit_body(&mut self, new_body: &'a str) {
        self.body = new_body;
        self.updated_at = Utc::now();
    }
}

struct Tag<'a> {
    title: &'a str,
    description: Option<&'a str>,
    color: Color,
}

struct Duration {
    // start_date: datetime
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}
impl Duration {

    fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Duration {
        Duration {
            start_date: start_date,
            end_date: end_date,
        }
    }

    fn get_duration_days(&self) -> u32 {
        let duration = self.end_date.signed_duration_since(self.start_date);
        duration.num_days() as u32
    }

    fn get_duration_weeks(&self) -> u32 {
        let duration = self.end_date.signed_duration_since(self.start_date);
        duration.num_weeks() as u32
    }

    fn remaining_days_from_now(&self) -> Option<u32> {
        let duration = self.end_date.signed_duration_since(Utc::now());
        //check if its not ended yet
        if duration.num_days() < 0 {
            None
        } else {
            Some(duration.num_days() as u32)
        }
    }

    fn remaining_weeks_from_now(&self) -> Option<u32> {
        let duration = self.end_date.signed_duration_since(Utc::now());
        //check if it is not ended yet
        if duration.num_days() < 0 {
            None
        } else {
            Some(duration.num_weeks() as u32)
        }
    }
}

#[derive(Debug, Clone)]
struct TodoItem<'a> {
    title: &'a str,
    is_completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>
}


impl<'a> TodoItem<'a> {
    fn new(title: &'a str) -> TodoItem<'a> {
        TodoItem {
            title: title,
            is_completed: false,
            created_at: Utc::now(),
            completed_at: None
        }
    }
    fn set_completed(&mut self){
        self.is_completed = true;
        self.completed_at = Some(Utc::now());
    }
    //TODO: add edit title method and figure out a way to update the parent todolist of the change
}

struct TodoList<'a> {
    title: &'a str,
    items: Vec<TodoItem<'a>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

impl<'a> TodoList<'a> {
    fn new(title: &'a str) -> TodoList<'a>{
        TodoList {
            title: title,
            items: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }
    fn add_todo_item(&mut self, title: &'a str){
        //TODO: make a test for this method
        let item = TodoItem::new(title);
        self.items.push(item);
        self.updated_at = Utc::now();
    }
    //TODO: add delete item


    //TODO: Add tests 
    fn get_completed_items(&self) -> Vec<TodoItem> {
        let items = self.items.clone(); 
       let mut completed_items = items.into_iter().filter(|item| item.is_completed).collect::<Vec<TodoItem>>();
       completed_items   
    }
    fn get_uncompleted_items(&self) -> Vec<TodoItem> {
        let items = self.items.clone(); 
       let mut completed_items = items.into_iter().filter(|item| !item.is_completed).collect::<Vec<TodoItem>>();
       completed_items   
    }

}

struct Project<'a> {
    title: &'a str,
    description: Option<&'a str>,
    notes: Vec<Note<'a>>,
    todo_lists: Vec<TodoList<'a>>,
    duration: Option<Duration>,
    priority: Option<Priority>,
    tags: Vec<Tag<'a>>,
    status: Option<Status>,
}

impl<'a> Project<'a> {
    fn new(title: &'a str) -> Project<'a> {
        Project {
            title: title,
            description: None,
            notes: Vec::new(),
            todo_lists: Vec::new(),
            duration: None,
            priority: None,
            tags: Vec::new(),
            status: Some(Status::Todo),
        }
    }
    fn description<'b>(&'b mut self, desc: &'a str) -> &'b mut Project<'a> {
        self.description = Some(desc);
        self
    }

    fn duration(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> &mut Project<'a> {
        self.duration = Some(Duration::new(start_date, end_date));
        self
    }

    fn priority(&mut self, priority: Priority) -> &mut Project<'a> {
        self.priority = Some(priority);
        self
    }

    fn add_tag(&mut self, tag: Tag<'a>) -> &mut Project<'a> {
        self.tags.push(tag);
        self
    }

    fn add_note(&mut self, note: Note<'a>) -> &mut Project<'a> {
        self.notes.push(note);
        self
    }

    fn add_todo_list(&mut self, todo_list: TodoList<'a>) -> &mut Project<'a> {
        self.todo_lists.push(todo_list);
        self
    }

    // set_status()
}

fn main() {
    let mut p = Project::new("test project");
    p.description("test desc").priority(Priority::High);

    dbg!(p.title);
    dbg!(p.description);
    dbg!(p.priority);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;

    //test Duration type
    #[test]
    fn test_get_duration_days() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Jan 30 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        assert_eq!(d.get_duration_days(), 29)
    }
    #[test]
    fn test_get_duration_weeks() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Jan 30 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        assert_eq!(d.get_duration_weeks(), 4)
    }

    #[test]
    fn test_remaining_days_from_now() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Dec 30 02:19:17 2110", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        let remaining_days = utc.datetime_from_str(&"Dec 30 02:19:17 2110", "%b %d %H:%M:%S %Y")
        .unwrap().signed_duration_since(Utc::now()).num_days() as u32;


        assert_eq!(d.remaining_days_from_now(), Some(remaining_days)); //really long date so I don't have to update it
    }

    #[test]
    fn test_remaining_days_from_now_passed_date() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Jan 30 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        assert_eq!(d.remaining_days_from_now(), None);
    }

    #[test]
    fn test_remaining_weeks_from_now() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Dec 30 02:19:17 2110", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        let remaining_weeks = utc.datetime_from_str(&"Dec 30 02:19:17 2110", "%b %d %H:%M:%S %Y")
        .unwrap().signed_duration_since(Utc::now()).num_weeks() as u32;

        assert_eq!(d.remaining_weeks_from_now(), Some(remaining_weeks));
    }

    #[test]
    fn test_remaining_weeks_from_now_passed_date() {
        let utc = Utc;
        let d = Duration::new(
            utc.datetime_from_str(&"Jan 1 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
            utc.datetime_from_str(&"Jan 30 02:19:17 2020", "%b %d %H:%M:%S %Y")
                .unwrap(),
        );
        assert_eq!(d.remaining_weeks_from_now(), None); //really long date so I don't have to update it
    }

    //test Note type
    #[test]
    fn test_edit_note_title() {
        let mut n = Note::new("test", "test body");
        //values that should change
        let old_title = n.title;
        let old_updated_at = n.updated_at;

        //values that should not change
        let old_body = n.body;
        let old_created_at = n.created_at;

        n.edit_title("test title");

        //test changed values
        assert_ne!(n.title, old_title);
        assert_eq!(n.title, "test title");

        assert_ne!(n.updated_at, old_updated_at);
        assert_eq!(n.updated_at.year(), Utc::now().year());
        assert_eq!(n.updated_at.month(), Utc::now().month());
        assert_eq!(n.updated_at.day(), Utc::now().day());
        assert_eq!(n.updated_at.hour(), Utc::now().hour());
        assert_eq!(n.updated_at.minute(), Utc::now().minute());
        // I will count that is fast enough to happen in the same second for now until I find a better way
        assert_eq!(n.updated_at.second(), Utc::now().second()); //TODO: please find a better way

        //test unchanged values
        assert_eq!(n.body, old_body);
        assert_eq!(n.created_at, old_created_at);
    }

    #[test]
    fn test_edit_note_body() {
        let mut n = Note::new("test title", "test");
        //values that should change
        let old_body = n.body;
        let old_updated_at = n.updated_at;

        //values that should not change
        let old_title = n.title;
        let old_created_at = n.created_at;

        n.edit_body("test body");

        //test changed values
        assert_ne!(n.body, old_body);
        assert_eq!(n.body, "test body");

        assert_ne!(n.updated_at, old_updated_at);
        assert_eq!(n.updated_at.year(), Utc::now().year());
        assert_eq!(n.updated_at.month(), Utc::now().month());
        assert_eq!(n.updated_at.day(), Utc::now().day());
        assert_eq!(n.updated_at.hour(), Utc::now().hour());
        assert_eq!(n.updated_at.minute(), Utc::now().minute());
        // I will count that is fast enough to happen in the same second for now until I find a better way
        assert_eq!(n.updated_at.second(), Utc::now().second()); //TODO: please find a better way

        //test unchanged values
        assert_eq!(n.title, old_title);
        assert_eq!(n.created_at, old_created_at);
    }


    //test todo item
    #[test]
    fn test_todo_item_set_completed(){
        let mut item = TodoItem::new("do something");
        //values should not be changed
        let old_title = item.title;
        let old_created_at = item.created_at;

        //values should be changed
        let old_is_completed = item.is_completed;

        item.set_completed();


        //test values should be changed
        assert_ne!(item.is_completed, old_is_completed);
        assert_eq!(item.is_completed, true);

        assert_ne!(item.completed_at, None);
        assert_eq!(item.completed_at.unwrap().year(), Utc::now().year());
        assert_eq!(item.completed_at.unwrap().month(), Utc::now().month());
        assert_eq!(item.completed_at.unwrap().day(), Utc::now().day());
        assert_eq!(item.completed_at.unwrap().hour(), Utc::now().hour());
        assert_eq!(item.completed_at.unwrap().minute(), Utc::now().minute());
        // I will count that is fast enough to happen in the same second for now until I find a better way
        assert_eq!(item.completed_at.unwrap().second(), Utc::now().second()); //TODO: please find a better way

        //test unchanged values
        assert_eq!(item.title, old_title);
        assert_eq!(item.created_at, old_created_at);

    }
}
