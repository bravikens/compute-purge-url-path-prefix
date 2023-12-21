use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Grab the URL
    let url_path = req.get_path().to_string();

    // Send the request to the origin    
    let mut beresp = req.send("origin_0")?;

    // Load up the current Surrogate-Key value (if there is one)
    let mut keys = beresp.get_header_str("Surrogate-Key").unwrap_or_default().to_string();

    //Split the URL by /
    let url_parts = url_path[1..url_path.len()].split("/");

    // Keep running track of the path
    let mut running_path = "/".to_string();
    let mut delimiter = "";
    // Loop through all of the URL parts
    for url_part in url_parts {
        running_path.push_str(delimiter);
        running_path.push_str(url_part);
        // Add the partial path to the surrogate keys array
        keys += &format!(" {}", running_path);
        delimiter = "/";
    }

    // Update the Surrogate-Key header with the new keys
    beresp.set_header("Surrogate-Key", keys.trim());

    // Log the surrogate key
    println!("Surrogate Keys: '{}'", keys.trim());

    // Send response to client
    Ok(beresp)
}
