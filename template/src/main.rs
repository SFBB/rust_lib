use std::fmt::Debug;
use data_impl_derive::data_functions;

#[derive(Debug)]
struct Template<T: Debug>
{
	data: T,
}

#[derive(Debug)]
#[data_functions]
struct DataTemplate<T: Debug>
{
	data_: T,
    version: u32,
}

impl <T: Debug> DataTemplate<T>
{
    fn new(data: T) -> Self {
        DataTemplate { data_: data, version: 1 }
    }

    fn increment_version(&mut self) {
        self.version += 1;
    } 
}

fn print_data<T>(data: &DataTemplate<T>)
    where T: std::fmt::Debug
{
	println!("{:?}", data);
}

fn modify_data<T>(data: &mut DataTemplate<T>)
    where T: std::fmt::Debug
{
    data.increment_version();
}

fn main() {
    println!("Hello, world!");

     // 1. Create a concrete instance of dataTemplate with i32
    let inner_instance: Template<i32> = Template { data: 123 };

    // 2. Create a concrete instance of Template with dataTemplate<i32>
    let mut outer_instance: DataTemplate<Template<i32>> = DataTemplate::new(inner_instance);

    // 3. Call printData with the concrete instance
    print_data::<Template<i32>>(&outer_instance);
    // 4. Call modifyData with the concrete instance
    modify_data(&mut outer_instance);
    // 5. Call printData again to see the changes
    print_data(&outer_instance);
    modify_data(&mut outer_instance);
    print_data(&outer_instance);

    outer_instance.modify_data();
    outer_instance.print_data();
    outer_instance.modify_data();
    outer_instance.print_data();
    outer_instance.modify_data();
    outer_instance.print_data();
}
