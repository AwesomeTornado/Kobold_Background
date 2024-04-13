use std::string::String;
use std::{env, };
use std::os::raw::c_float;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct KoboldResponse {
    id: String,
    kudos: c_float
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    println!("Hello, Kobold_background developers!");

    //first and foremost, work to retrieve the users api key, so that prompts are speedy.
    let key_name = "Kobold_BG_api_Key";
    let mut api_key = "0000000000".parse::<String>().unwrap();
    let key_set = !env::var(key_name).is_err();

    if(key_set){
      //  api_key = env::var(key_name).expect("0000000000");
    }else {
        std::println!("Can't find your API key in any env variables, Please set the path variable \"Kobold_BG_api_Key\" to your koboldai api key\n Thanks");
        panic!("Path variable not set.");
    }

    let client = reqwest::Client::new();
    let res = client.post("https://aihorde.net/api/v2/generate/text/async")
        .body("{\"prompt\":\"### Instruction:Come up with one or two words for a genre of literature, media, or music, then write a five sentence description of an image from that genre. Make sure your response focuses on the elements in an image from that genre. There should be vibrant description, and good imagery, of at least one paragraph. describe the image well. Describe individual elements of the image, including objects and colors.### Response:\",\"params\":{\"n\":1,\"max_context_length\":1024,\"max_length\":80,\"rep_pen\":1.08,\"temperature\":0.5,\"top_p\":0.92,\"top_k\":0,\"top_a\":0,\"typical\":1,\"tfs\":1,\"rep_pen_range\":256,\"rep_pen_slope\":0.7,\"sampler_order\":[6,0,1,3,4,2,5],\"use_default_badwordsids\":false,\"stop_sequence\":[\"### Instruction:\",\"### Response:\"],\"min_p\":0,\"dynatemp_range\":0,\"dynatemp_exponent\":1,\"smoothing_factor\":0},\"workers\":[]}")
        .header("apikey", api_key)
        .header("content-type", "application/json")
        .header( "content-length", "827")
        .send()
        .await.unwrap();

    //below is the unwrapping and error handling of the server response.
    let res_buffer = res.text().await;
    let message_id;
    match res_buffer {
        Ok(_) => message_id = res_buffer.unwrap(),
        Err(_) => message_id = "webreq failed, server returned unexpected result.".parse::<String>().unwrap()
    }
    println!("{}", message_id);
    let message_json: KoboldResponse = serde_json::from_str(&*message_id).expect("json unwrapping error, likely bad webreq");
    let message_id = message_json.id;
    println!("{}", message_id);

    //at this point, we have the message response id, and need to wait for the response to be generated.


    Ok(())

}
