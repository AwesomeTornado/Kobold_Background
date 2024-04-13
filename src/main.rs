use std::string::String;
use std::{env, thread};
use std::os::raw::c_float;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct TextGenInitResponse {
    id: String,
    kudos: c_float
}
#[derive(Serialize, Deserialize)]
struct TextGenStatus {
    generations: Generations,
    finished: bool,
    processing: bool,
    restarted: bool,
    waiting: bool,
    done: bool,
    faulted: bool,
    wait_time: i32,
    queue_position: i32,
    kudos: c_float,
    is_possible: bool
}
#[derive(Serialize, Deserialize)]
struct TextGenStatusNOGEN {
    finished: i8,
    processing: i8,
    restarted: i8,
    waiting: i8,
    done: bool,
    faulted: i8,
    wait_time: i32,
    queue_position: i32,
    kudos: c_float,
    is_possible: bool
}
#[derive(Serialize, Deserialize)]
struct Generations {
    text: String,
    seed: i32,
    gen_metadata: Nil,
    worker_id: String,
    worker_name: String,
    model: String,
    state: String
}
#[derive(Serialize, Deserialize)]
struct Nil {
}

async fn get_message_id(api_key: String) -> String{
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
    let message_json: TextGenInitResponse = serde_json::from_str(&*message_id).expect("json unwrapping error, likely bad webreq");
    let message_id = message_json.id;
    println!("{}", message_id);
    return message_id;
}

async fn get_message_status(message_id: String) -> bool{
    let url =  "https://aihorde.net/api/v2/generate/text/status/".to_owned() + &*message_id;

    let client = reqwest::Client::new();
    let res = client.get(url)
        .send()
        .await.unwrap();

    //below is the unwrapping and error handling of the server response.
    let res_buffer = res.text().await;
    let message_status;
    match res_buffer {
        Ok(_) => message_status = res_buffer.unwrap(),
        Err(_) => message_status = "webreq failed, server returned unexpected result.".parse::<String>().unwrap()
    }
    println!("{}", message_status);
    let message_json: TextGenStatusNOGEN = serde_json::from_str(&*message_status).expect("json unwrapping error, likely bad webreq");
    let status = message_json.done;
    println!("{}", status);
   return status;
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

    let message_id = get_message_id(api_key).await;
    println!("get_message_id has been run, result of function is:\n{}", message_id);
    //at this point, we have the message response id, and need to wait for the response to be generated.
    let second = std::time::Duration::from_millis(1000);
    while true {
        thread::sleep(second);
        get_message_status(message_id.clone()).await;
    }


    Ok(())

}
