mod tasks;

fn main() {
    let task1 = tasks::Task::new(
        "Task pierwszy".into(),
        "Tu jest jego opis".into(),
        "Wazne".into(),
        Option::None,
        tasks::Status::Todo,
        10
    ).unwrap();

    task1.print();

    let task2 = tasks::Task::new_from_user();

    match task2 {
        Ok(task) => task.print(),
        Err(_) => println!("Something went wrong")
    }

}
