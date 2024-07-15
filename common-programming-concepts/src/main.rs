fn main() {
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", plus_one(y));
}

fn _another_function() {
    println!("Another function");
}

fn _variables_mutability() {
    let spaces: &str = "   ";
    {
        let spaces: usize = spaces.len();
        println!("The value of spaces is: {}", spaces);
    }
    println!("The value of spaces is: {}", spaces);
}

fn _data_types() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    let marks: (u32, i32, f32) = (300, -30, 100.212);

    let threehundread = marks.0;
    println!("{threehundread}");

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let jan: &str = months[0];

    println!("{jan}");

    let numbers: [i32; 50] = [1; 50];

    for i in numbers {
        println!("{i}")
    }
}

fn _out_of_bound_array_accessing() {
    let numbers2 = [1, 2, 3, 4, 5];

    println!("Enter an array index");

    let mut index: String = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read number");

    let index: usize = index.trim().parse().expect("Not a number");

    let userchoice = numbers2[index];

    println!("{userchoice}");
}
