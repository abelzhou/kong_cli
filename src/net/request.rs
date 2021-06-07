
use ureq::Response;

pub fn get(url: &String) -> Result<String, ureq::Error> {
    // println!("Execute get url {}", url);
    let resp =  ureq::get(url).call()?.into_string()?;
    Ok(resp)
}

pub fn post(url: &String, data: &[(&str, &str)]) -> Result<Response, ureq::Error>{
    // println!("Execute post url {}", url); 
    let resp = ureq::post(url).send_form(data)?;
    return Ok(resp);
}
