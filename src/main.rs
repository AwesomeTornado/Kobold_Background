use std::string::String;
use std::{env, fs};
use std::os::raw::c_float;
use std::path::Path;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use wallpaper_windows_user32;
use std::{fs::File, io::{copy, Cursor}};
use std::time::{SystemTime};
use anyhow::{Result};
use core::time::Duration;
use rand::Rng;

async fn return_genre() -> String {
    let mut rng = rand::thread_rng();
    let genres = [
    "Poetry",
    "Drama",
    "Mystery",
    "Fantasy",
    "Science Fiction",
    "Romance",
    "Horror",
    "Thriller",
    "Comedy",
    "Satire",
    "Historical Fiction",
    "Adventure",
    "Biography",
    "Autobiography",
    "Memoir",
    "Young Adult (YA)",
    "Children's Literature",
    "Bildungsroman (Coming-of-age)",
    "Dystopian",
    "Utopian",
    "Absurdist",
    "Magical Realism",
    "Realistic Fiction",
    "Paranormal",
    "Mythology",
    "Fairy Tale",
    "Fable",
    "Folklore",
    "Epic",
    "Tragedy",
    "Comedy of Manners",
    "Metafiction",
    "Experimental Literature",
    "Satirical Fiction",
    "Legal Thriller",
    "Psychological Thriller",
    "Espionage",
    "Portrait",
    "Landscape",
    "Still Life",
    "Abstract",
    "Surrealism",
    "Impressionism",
    "Cubism",
    "Expressionism",
    "Realism",
    "Pop Art",
    "Conceptual Art",
    "Street ",
    "Fashion ",
    "Wildlife ",
    "Documentary ",
    "Macro ",
    "Black and White ",
    "High Dynamic Range (HDR) ",
    "Panoramic ",
    "Aerial ",
    "Astro",
    "Food ",
    "Sports ",
    "Architectural ",
    "Event ",
    "Nature ",
    "Fine Art ",
    "Baroque",
    "Renaissance",
    "Rococo",
    "Neoclassical",
    "Romanticism",
    "Abstract Expressionism",
    "Minimalism",
    "Postmodernism",
    "Art Deco",
    "Art Nouveau",
    "Gothic",
    "Pre-Raphaelite",
    "Fauvism",
    "Dadaism",
    "Constructivism",
    "Pointillism",
    "Symbolism",
    "Cubo-Futurism",
    "Neo-Impressionism",
    "Social Realism",
    "Abstract Art",
    "Hyperrealism",
    "Street Art",
    "Photorealism"
        ];
    let string: String = genres[rng.gen_range(0..genres.len())].parse().unwrap();
    return string;
}

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
        let genre = return_genre().await;
        body = ("{\"prompt\":\"### Instruction:Write a description of an image stemming from the genre of ".to_owned() + &*genre + &*". Make sure your response focuses on the elements in an image from ".to_owned() + &*genre + &*". There should be vibrant description, and good imagery, of at least one paragraph. describe the image well. Describe individual elements of the image, including objects and colors. DO NOT INCLUDE PEOPLE. DO NOT INCLUDE WOMEN. DO NOT INCLUDE EXPLICIT CONTENT. be as descriptive about ".to_owned() + &*genre + &*" as possible. ### Response:\",\"params\":{\"n\":1,\"max_context_length\":1024,\"max_length\":80,\"rep_pen\":1.08,\"temperature\":0.5,\"top_p\":0.92,\"top_k\":0,\"top_a\":0,\"typical\":1,\"tfs\":1,\"rep_pen_range\":256,\"rep_pen_slope\":0.7,\"sampler_order\":[6,0,1,3,4,2,5],\"use_default_badwordsids\":false,\"stop_sequence\":[\"### Instruction:\",\"### Response:\"],\"min_p\":0,\"dynatemp_range\":0,\"dynatemp_exponent\":1,\"smoothing_factor\":0},\"workers\":[]}").parse().unwrap();
        url = "https://aihorde.net/api/v2/generate/text/async";
    }else if selector == "image" {
        body = "{\"prompt\":\"".to_owned() + &*prompt.replace("\n", "").replace("\"","\\\"") +  "beautiful, sharp, clear, focused ### People, persons, women, nsfw, explicit, realistic, easyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame , worst quality, low quality:1.4), EasyNegative, bad anatomy, bad hands, cropped, missing fingers, missing toes, too many toes, too many fingers, missing arms, long neck, Humpbacked, deformed, disfigured, poorly drawn face, distorted face, mutation, mutated, extra limb, ugly, poorly drawn hands, missing limb, floating limbs, disconnected limbs, malformed hands, out of focus, long body, monochrome, symbol, text, logo, door frame, window frame, mirror frame\",\"params\":{\"cfg_scale\":7.5,\"seed\":\"\",\"sampler_name\":\"k_euler_a\",\"height\":1088,\"width\":1920,\"post_processing\":[],\"steps\":40,\"tiling\":false,\"karras\":true,\"hires_fix\":false,\"clip_skip\":1,\"n\":1},\"nsfw\":false,\"censor_nsfw\":true,\"trusted_workers\":true,\"models\":[\"AlbedoBase XL (SDXL)\"],\"r2\":true,\"replacement_filter\":true,\"shared\":false,\"slow_workers\":true,\"dry_run\":false}";        url = "https://aihorde.net/api/v2/generate/async";
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
    if image_status.contains("censor"){
        image_json.faulted = true;
        return image_json;
    }
    image_json = serde_json::from_str(&*image_status).expect("json unwrapping error, likely bad webreq");
    return image_json;
}

async fn download_image_to(url: &str, file_name: &str) -> Result<()> {
    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await?;
    // Create a new file to write the downloaded image to
    let mut file = File::create(file_name)?;

    // Create a cursor that wraps the response body
    let mut content =  Cursor::new(response.bytes().await?);
    // Copy the content from the cursor to the file
    copy(&mut content, &mut file)?;

    Ok(())
}

async fn cache_desktop_background(api_key: String, archive_dir: &str) -> String{
    let message_id = get_horde_id(api_key.clone(), "text", "").await;
    println!("get_message_id has been run, result of function is:\n{}", message_id);
    //at this point, we have the message response id, and need to wait for the response to be generated.
    let second = Duration::from_millis(10000);
    let mut done = false;
    //I don't want to reinitalize an entire blank textgenstatus object, so I use the failed one from
    //the get message status function. Lazy, but understandable and fast.
    let mut final_prompt: TextGenStatus = get_message_status(message_id.clone()).await;
    while !done {
        async_std::task::sleep(second).await;
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
    //I don't want to reinitalize an entire blank imagegenstatus object, so I use the failed one from
    //the get message status function. Lazy, but understandable and fast.
    let mut final_image: ImageGenStatus = get_image_status(image_id.clone()).await;
    while !done {
        async_std::task::sleep(second).await;
        final_image = get_image_status(image_id.clone()).await;
        done = final_image.done;
        if final_image.faulted == true{
            break;
        }
    }

    if done{
        println!("finally complete! We have located your image, and will send you the link now.");
        let img_url =  final_image.generations[0].img.to_string();
        println!("{}", img_url);

        let file = archive_dir.to_owned() + &*image_id + ".webp";
        download_image_to(&*img_url, &*file).await.expect("Error downloading image");
        return file;
    }
    return "NSFW".to_string();
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    println!("Hello, Kobold_background developers!");

    //first and foremost, work to retrieve the users api key, so that prompts are speedy.
    let key_name = "Kobold_BG_api_Key";
    let api_key:String;
    let key_set = !env::var(key_name).is_err();
    let archive_dir = "C:/Kobold_Backgrounds/";
    let delay:Duration;

    //secondly, ensure that there is a location to store images, and permissions are set
    if !Path::new(archive_dir).exists() {
        println!("Some directories need to be created, as this is the first install of this application...");
        fs::create_dir_all(archive_dir).unwrap();
    }

    if key_set {
        //comment out this line to use the free api key. (only for debug purposes)
        let environment_var = env::var(key_name).expect("0000000000");
        let both_configs = environment_var.split(",").collect::<Vec<&str>>();
        api_key = both_configs[0].to_string();
        println!("{}", api_key);
        delay = Duration::from_millis(both_configs[1].parse().unwrap());
    }else {
        std::println!("Can't find your API key in any env variables, Please set the path variable \"Kobold_BG_api_Key\" to your koboldai api key\n Thanks");
        panic!("Path variable not set.");
    }

    let mut start_time;
    let mut end_time;
    let mut file;
    loop {
        start_time = SystemTime::now();
        end_time = start_time.checked_add(delay).unwrap();
        file = cache_desktop_background(api_key.clone(), archive_dir).await;
        while file != "NSFW"{
            file = cache_desktop_background(api_key.clone(), archive_dir).await;
        }
        async_std::task::sleep(end_time.duration_since(SystemTime::now()).expect("excessive wait time caused crash")).await;
        wallpaper_windows_user32::set(file).expect("Error when setting desktop background.");
    }

    //Ok(())

}
