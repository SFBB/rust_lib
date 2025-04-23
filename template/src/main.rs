use std::fmt::Debug;

#[derive(Debug)]
struct Templat<T: Debug>
{
	data: T,
}

#[derive(Debug)]
struct DataTemplate<T: Debug>
{
	data_: T,
}

fn print_data<T>(data: T)
    where T: std::fmt::Debug
{
	println!("{:?}", data);
}


fn main() {
    println!("Hello, world!");

     // 1. Create a concrete instance of dataTemplate with i32
    let inner_instance: DataTemplate<i32> = DataTemplate { data_: 123 };

    // 2. Create a concrete instance of Template with dataTemplate<i32>
    let outer_instance: Templat<DataTemplate<i32>> = Templat { data: inner_instance };

    // 3. Call printData with the concrete instance
    print_data(outer_instance);
}
