use std::error::Error;

use crate::{db::view_request::ViewRequest, player::animation::Animation};

pub fn make_animation_request(title: String) -> Result<Option<Animation>, Box<dyn Error>> {
    let animations = get_all_animations()?;

    let matching = animations
        .iter()
        .find(|ani| ani.title == title)
        .map(|x| x.clone());

    return Ok(matching);
}

pub fn get_all_animations() -> Result<Vec<Animation>, Box<dyn Error>> {
    let resp = reqwest::blocking::get(format!(
        "http://localhost:8090/api/collections/Animations/records/"
    ))?;

    if resp.status() != 200 {
        println!("Could not find record");
    } else {
        let body = resp.text()?;
        let view: ViewRequest = ViewRequest::from(&body)?;
        let animations: Vec<Animation> = view.items;
        return Ok(animations);
    }

    Ok(vec![])
}
