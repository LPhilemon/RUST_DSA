mod enumerate {

    #[derive(Debug)]
    pub enum Seasons {
        Spring,
        Summer,
        Winter,
        Autumn,
    }

    impl Seasons {
        pub fn water(season: Seasons) {
            match season {
                Seasons::Spring => println!("it is: spring"),
                Seasons::Summer => println!("it is: summer"),
                Seasons::Winter => println!("it is: winter"),
                Seasons::Autumn => println!("it is: autumn"),
            }
        }
    }
}

fn main() {
    let seas = Seasons::Spring;

    Seasons::water(seas);
}

