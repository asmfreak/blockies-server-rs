#[macro_use] extern crate rocket;
use rocket::form::FromFormField;
use rocket::serde::Deserialize;
use rocket::http::{Status, ContentType};
use std::io;
use blockies::{Classic,Ethereum,Error};

#[derive(FromFormField, Debug)]
enum Algorithm{
    Classic, Ethereum
}

enum AlgorithmImpl{
    Classic(Classic), Ethereum(Ethereum)
}

impl AlgorithmImpl{
    pub fn from_args(
            algorithm:Option<Algorithm>, size: Option<usize>, scale: Option<usize>
    ) -> Self{
        let size = size.unwrap_or(8);
        let scale =scale.unwrap_or(16);
        match algorithm.unwrap_or(Algorithm::Classic){
            Algorithm::Classic => Self::Classic(Classic{ 
                size: size, scale: scale, 
                color: None, background_color: None,
            }),
            Algorithm::Ethereum => Self::Ethereum(Ethereum{ 
                size: size, scale: scale, 
                color: None, background_color: None, spot_color: None
            })
        }
    }
    pub fn create_icon<W: io::Write>(&self, writer: W, seed: &[u8]) -> Result<(), Error> {
        match self {
            Self::Classic(i) => i.create_icon(writer, seed),
            Self::Ethereum(i) => i.create_icon(writer, seed)
        }
    }
}

#[get("/<seed>?<algorithm>&<size>&<scale>")]
fn index(
        seed:&str, algorithm: Option<Algorithm>, size: Option<usize>, scale: Option<usize>
) -> Result<(ContentType, Vec<u8>), Status> {
    let blockies = AlgorithmImpl::from_args(algorithm, size, scale);
    let mut png = Vec::new();
    match blockies.create_icon(&mut png, seed.as_bytes()) {
        Ok(()) => Ok((ContentType::PNG, png)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        url_base: Option<String>
    }

    let cfg : Config = figment.extract().expect("config");
    rocket.mount(cfg.url_base.unwrap_or("/".to_string()), routes![index])
}

