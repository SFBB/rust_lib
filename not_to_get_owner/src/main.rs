use std::sync::Arc;
use std::sync::Mutex;

struct MyType {
    value: i32,
}

struct GenericType<t>
where
    t: Send + Sync,
{
    value: i32,
    data: t,
}

struct SubManager {
    value: i32,
}
impl SubManager {
    pub fn new() -> Self {
        SubManager { value: 9 }
    }
    pub fn run(&self, manager: Arc<Mutex<SubManager>>) {
        println!("manager.value: {}", manager.lock().unwrap().value);
    }
}
struct manager {
    value: i32,
    sub_manager: SubManager,
    another_sub_manager: Arc<Mutex<SubManager>>,
}

impl manager {
    pub fn new() -> Self {
        manager {
            value: 9,
            sub_manager: SubManager::new(),
            another_sub_manager: Arc::new(Mutex::new(SubManager::new())),
        }
    }

    pub fn run(&self) {
        self.sub_manager.run(self.another_sub_manager.clone());
    }
}

async fn generic_print<t: Send + Sync>(data: GenericType<t>) {
    tokio::spawn(async move {
        println!("data.value: {}", data.value);
    })
    .await;
}
#[tokio::main]
async fn main() {
    println!("hello, world!");

    let var = MyType { value: 6 };

    let generic_var = GenericType {
        value: 9,
        data: var,
    };
    generic_print(generic_var).await;
    let manager = manager::new();
    manager.run();
}
