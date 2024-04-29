use std::error::Error;

use crate::log::logger::log;

pub fn make_animation_request(title: String) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("http://localhost:8090/api/collections/Animations/records/15504u7izozzgea")?;

    log(format!("Making Animation Request, {}", title).as_str());

    if resp.status() != 200 {
        println!("could not find record");
    }

    else {
        let body = resp.text()?;
        println!("{}", body);
    }

    Ok(())
}
