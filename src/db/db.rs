use std::error::Error;

use crate::{db::view_request::ViewAnimationRequest, player::animation::Animation, tree::tree::Tree};

use super::view_request::ViewTreeRequest;

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
        let view: ViewAnimationRequest = ViewAnimationRequest::from(&body)?;
        let animations: Vec<Animation> = view.items;
        return Ok(animations);
    }

    Ok(vec![])
}

pub fn get_all_trees() -> Result<Vec<Tree>, Box<dyn Error>> {
    let resp = reqwest::blocking::get(format!(
            "http://localhost:8090/api/collections/Tree/records/"
            ))?;

    if resp.status() != 200 {
        println!("Could not find record");
    } else {
        let body = resp.text()?;
        let view: ViewTreeRequest = ViewTreeRequest::from(&body)?;
        let trees: Vec<Tree> = view.items;
        return Ok(trees);
    }
    Ok(vec![])
}
