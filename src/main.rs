use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIResponse {
    choices: Vec<OAIChoices>,
}

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16, // max tokens you want in your respons
} // Struct for the OpenAI API request

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let https = HttpsConnector::new();

    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Generate a sql code for the sentence";
    let oai_token = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> "); // println flushes it, print doesn't
        stdout().flush().unwrap(); // flush stdout
        let mut user_text = String::new(); // variable to store user input
        let sp = Spinner::new(&Spinners::Dots12, "\t\tSqlGPT is thinking...".into());
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text), // formatting the prompt
            max_tokens: 1000, // max tokens in the response, 100 to see the entire response
        };
        let body = Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json") // In requests, (such as POST or PUT), the client tells the server what type of data is being sent.
            .header("Authorization", &auth_header_val) // takes in a key and a value
            .body(body) // passing in the body
            .unwrap(); // if value is none, panic
        let res = client.request(req).await?;

        // getting the body of the response, storing it in the variable body
        let body = hyper::body::aggregate(res).await?; // Aggregate the data buffers from a body asynchronously (hence await). Waiting for all the chunks of data to come back and pull a body out of that.

        // deserializing the body into the OpenAI API response struct
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        // we've got response by now, stop the spinner

        // stopping the spinner
        sp.stop();

        println!(""); // println!: same as print! but a newline is appended.

        // printing the response, choices is coming from the OpenAI API response struct, so we are accessing the choices vector and getting the first one
        // choices is a possible list of responses, we are getting the first one
        println!("{}", json.choices[0].text);
    }
    Ok(())
}
