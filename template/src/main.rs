#[derive(Debug)]
struct Templat<T>
{
	data: T,
}

#[derive(Debug)]
struct dataTemplate<T>
{
	data_: T,
}

fn printData<T>(data: T)
    where T: std::fmt::Debug
{
	println!("{:?}", data);
}


fn main() {
    println!("Hello, world!");

     // 1. Create a concrete instance of dataTemplate with i32
    let inner_instance: dataTemplate<i32> = dataTemplate { data_: 123 };

    // 2. Create a concrete instance of Template with dataTemplate<i32>
    let outer_instance: Templat<dataTemplate<i32>> = Templat { data: inner_instance };

    // 3. Call printData with the concrete instance
    printData(outer_instance);
}
