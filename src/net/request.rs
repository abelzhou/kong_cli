use std::collections::HashMap;

pub fn get(url: String) -> Result<String, ureq::Error> {
    println!("execute get url {}", &url);
    // let resp = reqwest::blocking::get(url)?
    // .json::<HashMap<String, String>>()?;
    // println!("resp : {:#?}", resp);
    // return Ok(resp);
    let resp =  ureq::get(&url).call()?.into_string()?;
    Ok(resp)
}
