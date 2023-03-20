use std::io;

fn main() {
    //vector to be rearranged
    let mut vector: Vec<i32> = Vec::new();

    let mut control: i32 = 0;

    println!(
        "




#THIS PROGRAM GETS USER INPUT OF VALUES THAT ARE ADDED TO A VECTOR AND SORTED USING TH BUBBLE SORT ALGORITHM#
"
    );
    loop {
        let mut controller = String::new();

        println!("Add value to vector(enter 'no' to stop):");
        io::stdin().read_line(&mut controller).expect("FAILED");

        let trimmed = controller.trim();

        match &trimmed as &str {
            "no" => {
                break;
            }
            _ => 0,
        };
        match trimmed.parse::<i32>() {
            Ok(num) => num,
            _ => {
                println!(
                    "
              Wrong value, enter a number!
              "
                );
                continue;
            }
        };
        control = trimmed.parse::<i32>().unwrap();

        vector.push(control);

        println!(
            "added:{}...current vector is {:?}
",
            trimmed, vector
        );
    }

    //vector.push(3);
    //vector.push(1);
    println!(
        "
#RESULTS#"
    );
    println!("before bubble sort: {:?}", vector);
    let mut outer_controller: usize = 0;
    let length: usize = vector.len();

    while outer_controller < length {
        let mut inner_controller: usize;

        inner_controller = outer_controller;
        inner_controller += 1;
        while inner_controller < length {
            if &vector[outer_controller] > &vector[inner_controller] {
                let holder = vector[outer_controller];
                vector[outer_controller] = vector[inner_controller];
                vector[inner_controller] = holder;
            }

            inner_controller += 1;
            // println!("innercontroller value: {}", inner_controller);
            //println!("lenght:{}, array:{:?}", vector.len(), vector);
        }
        outer_controller += 1;
        //println!("outercontroller value: {}", outer_controller);
    }
    //call function on the array--  -- print the rearranged array.
    println!("after_bubbling : {:?}", vector);
}
