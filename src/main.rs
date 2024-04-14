use std::string::String;
use std::{env, thread};
use std::os::raw::c_float;
use serde::{Deserialize, Serialize};
use reqwest::Error;

#[derive(Serialize, Deserialize)]
struct TextGenInitResponse {
    id: String,
    kudos: c_float
}
#[derive(Serialize, Deserialize)]
struct TextGenStatus {
    generations: One,
    finished: i8,
    processing: i8,
    restarted: i8,
    waiting: i8,
    done: bool,
    faulted: bool,
    wait_time: i32,
    queue_position: i32,
    kudos: c_float,
    is_possible: bool
}

#[derive(Serialize, Deserialize)]
struct TextGeneration {
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
    //nothing to see here
}
#[derive(Serialize, Deserialize)]
struct One {
    o: TextGeneration
}


#[derive(Serialize, Deserialize)]
struct ImageGeneration {
    pub img: String,
    pub seed: String,
    pub id: String,
    pub censored: bool,
    pub gen_metadata: Nil,
    pub worker_id: String,
    pub worker_name: String,
    pub model: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
struct ImageGenStatus {
    pub generations: Vec<ImageGeneration>,
    pub shared: bool,
    pub finished: i64,
    pub processing: i64,
    pub restarted: i64,
    pub waiting: i64,
    pub done: bool,
    pub faulted: bool,
    pub wait_time: i64,
    pub queue_position: i64,
    pub kudos: f64,
    pub is_possible: bool,
}

async fn get_horde_id(api_key: String, selector: &str, prompt: &str) -> String{
    let body:String;
    let url;
    if selector == "text" {
        body = "{\"prompt\":\"### Instruction:Come up with one or two words for a genre of literature, media, or music, then write a five sentence description of an image from that genre. Make sure your response focuses on the elements in an image from that genre. There should be vibrant description, and good imagery, of at least one paragraph. describe the image well. Describe individual elements of the image, including objects and colors.### Response:\",\"params\":{\"n\":1,\"max_context_length\":1024,\"max_length\":80,\"rep_pen\":1.08,\"temperature\":0.5,\"top_p\":0.92,\"top_k\":0,\"top_a\":0,\"typical\":1,\"tfs\":1,\"rep_pen_range\":256,\"rep_pen_slope\":0.7,\"sampler_order\":[6,0,1,3,4,2,5],\"use_default_badwordsids\":false,\"stop_sequence\":[\"### Instruction:\",\"### Response:\"],\"min_p\":0,\"dynatemp_range\":0,\"dynatemp_exponent\":1,\"smoothing_factor\":0},\"workers\":[]}".parse().unwrap();
        url = "https://aihorde.net/api/v2/generate/text/async";
    }else if selector == "image" {
        body = "{\"prompt\":\"".to_owned() + &*prompt.replace("\n", "") + "### , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame\",\"params\":{\"cfg_scale\":7.5,\"seed\":\"\",\"sampler_name\":\"k_euler_a\",\"height\":1088,\"width\":1920,\"post_processing\":[],\"steps\":40,\"tiling\":false,\"karras\":true,\"hires_fix\":false,\"clip_skip\":1,\"n\":1},\"nsfw\":false,\"censor_nsfw\":true,\"trusted_workers\":true,\"models\":[\"AlbedoBase XL (SDXL)\"],\"r2\":true,\"replacement_filter\":true,\"shared\":false,\"slow_workers\":true,\"dry_run\":false}";
        url = "https://aihorde.net/api/v2/generate/async";
    }else{
        panic!("no targets matched selector.");
    }
    println!("\n\n\n{}\n\n\n", body);
    let message_length = body.len();
    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(body)
        .header("apikey", api_key)
        .header("content-type", "application/json")
        .header( "content-length", message_length)
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
    if message_id.contains("KudosUpfront") {
        panic!("You are Poor!\nPlease acquire the required funds in Kudos in order to use this application.");
    }
    let message_json: TextGenInitResponse = serde_json::from_str(&*message_id).expect("json unwrapping error, likely bad webreq");
    let message_id = message_json.id;
    println!("{}", message_id);
    return message_id;
}

async fn get_message_status(message_id: String) -> TextGenStatus {
    let url =  "https://aihorde.net/api/v2/generate/text/status/".to_owned() + &*message_id;
    let mut message_json: TextGenStatus = TextGenStatus {
        generations: One {
            o: TextGeneration {
                text: "".to_string(),
                seed: 0,
                gen_metadata: Nil {},
                worker_id: "".to_string(),
                worker_name: "".to_string(),
                model: "".to_string(),
                state: "".to_string(),
            }
        },
        finished: 0,
        processing: 0,
        restarted: 0,
        waiting: 0,
        done: false,
        faulted: false,
        wait_time: 0,
        queue_position: 0,
        kudos: 0.0,
        is_possible: false,
    };
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
    if !message_status.contains("ok") {
        return message_json;
    }
    message_json = serde_json::from_str(&*message_status).expect("json unwrapping error, likely bad webreq");
    return message_json;
}

async fn get_image_status(image_id: String) -> ImageGenStatus{
    let url =  "https://aihorde.net/api/v2/generate/status/".to_owned() + &*image_id;
    let mut image_json: ImageGenStatus = ImageGenStatus {
        generations: vec![],
        shared: false,
        finished: 0,
        processing: 0,
        restarted: 0,
        waiting: 0,
        done: false,
        faulted: false,
        wait_time: 0,
        queue_position: 0,
        kudos: 0.0,
        is_possible: false,
    };
    let client = reqwest::Client::new();
    let res = client.get(url)
        .send()
        .await.unwrap();

    //below is the unwrapping and error handling of the server response.
    let res_buffer = res.text().await;
    let image_status;
    match res_buffer {
        Ok(_) => image_status = res_buffer.unwrap(),
        Err(_) => image_status = "webreq failed, server returned unexpected result.".parse::<String>().unwrap()
    }
    println!("{}", image_status);
    if !image_status.contains("ok") {
        return image_json;
    }
    image_json = serde_json::from_str(&*image_status).expect("json unwrapping error, likely bad webreq");
    return image_json;
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    println!("Hello, Kobold_background developers!");

    //first and foremost, work to retrieve the users api key, so that prompts are speedy.
    let key_name = "Kobold_BG_api_Key";
    let api_key;
    let key_set = !env::var(key_name).is_err();

    if key_set {
        //comment out this line to use the free api key. (only for debug purposes)
        api_key = env::var(key_name).expect("0000000000");
    }else {
        std::println!("Can't find your API key in any env variables, Please set the path variable \"Kobold_BG_api_Key\" to your koboldai api key\n Thanks");
        panic!("Path variable not set.");
    }

    let message_id = get_horde_id(api_key.clone(), "text", "").await;
    println!("get_message_id has been run, result of function is:\n{}", message_id);
    //at this point, we have the message response id, and need to wait for the response to be generated.
    let second = std::time::Duration::from_millis(10000);
    let mut done = false;
    //I don't want to reinitalize an entire blank textgenstatus object, so i use the failed one from
    //the get message status function. Lazy, but understandable and fast.
    let mut final_prompt: TextGenStatus = get_message_status(message_id.clone()).await;
    while !done {
        thread::sleep(second);
        final_prompt = get_message_status(message_id.clone()).await;
        done = final_prompt.done;
    }
    //we now have our final prompt in "final prompt"
    let text = final_prompt.generations.o.text;
    println!("{}", text);
    //I did it! we finally have a single prompt generation, ready to send to the dreamers of the horde.

    let image_id = get_horde_id(api_key.clone(), "image", &*text.clone()).await;
    println!("{}", image_id);

    done = false;
    //I don't want to reinitalize an entire blank imagegenstatus object, so i use the failed one from
    //the get message status function. Lazy, but understandable and fast.
    let mut final_image: ImageGenStatus = get_image_status(image_id.clone()).await;
    while !done {
        thread::sleep(second);
        final_image = get_image_status(image_id.clone()).await;
        done = final_image.done;
    }

    println!("finally complete! We have located your image, and will send you the link now.");
    println!("{}", final_image.generations[0].img.to_string());

    Ok(())

}
