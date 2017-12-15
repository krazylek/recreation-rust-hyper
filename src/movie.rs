use rand;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub title: String,
    pub year: usize,
    pub bad_guy: String
}

pub fn create_movie() -> Movie {
    let num = rand::thread_rng().gen_range(1, 3);
    match num {
        1 => Movie {
            title: "You Only Live Twice".to_owned(),
            year: 1967,
            bad_guy: "Blofeld".to_owned(),
        },
        2 => Movie { 
            title: "The Title".to_owned(),
            year: 2017, 
            bad_guy: "only me".to_owned()
        },
        _ => Movie {
            title: "empty".to_owned(),
            year: 9999, 
            bad_guy: "empty".to_owned()
        }
    }
}