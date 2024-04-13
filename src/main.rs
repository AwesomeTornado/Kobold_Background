use std::{env, };
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    println!("Hello, Kobold_background developers!");

    let key_name = "Kobold_BG_api_Key";
    let mut api_key;
    let key_set = env::var(key_name).is_err();

    if(!key_set){
        api_key = env::var(key_name).expect("0000000000");
    }else {
        std::println!("Can't find your API key in any env variables, Please set the path variable \"Kobold_BG_api_Key\" to your koboldai api key\n Thanks");
        panic!("Path variable not set.");
    }

    let client = reqwest::Client::new();
    let res = client.post("https://aihorde.net/api/v2/generate/text/async")
        .body("{\"prompt\":\"\n### Instruction:\nCome up with one or two words for a genre of literature, media, or music, then write a five sentence description of an image from that genre. Make sure your response focuses on the elements in an image from that genre. There should be vibrant description, and good imagery, of at least one paragraph. describe the image well. Describe individual elements of the image, including objects and colors.\n### Response:\n\",\"params\":{\"n\":1,\"max_context_length\":1024,\"max_length\":80,\"rep_pen\":1.11,\"temperature\":0.5,\"top_p\":1,\"top_k\":0,\"top_a\":0,\"typical\":1,\"tfs\":1,\"rep_pen_range\":256,\"rep_pen_slope\":0.7,\"sampler_order\":[6,0,1,2,3,4,5],\"use_default_badwordsids\":false,\"stop_sequence\":[\"### Instruction:\",\"### Response:\"],\"min_p\":0,\"dynatemp_range\":0,\"dynatemp_exponent\":1,\"smoothing_factor\":0},\"models\":[\"aphrodite/KoboldAI/LLaMA2-13B-Estopia\",\"aphrodite/KoboldAI/LLaMA2-13B-Estopia\",\"aphrodite/Sao10K/Fimbulvetr-11B-v2\",\"aphrodite/test-mistral2-7B-holodeck-v1\",\"koboldcpp/Fimbulvetr-11B-v2\",\"koboldcpp/LLaMA2-13B-Psyfighter2\",\"koboldcpp/LLaMA2-13B-Psyfighter2\",\"koboldcpp/mistral-7b-instruct-v0.2.Q5_K_M\",\"koboldcpp/Mistral-ClaudeLimaRP-v3-7B.q8_0\",\"koboldcpp/Noromaid-v0.4-Mixtral-Instruct-8x7b-Zloss\",\"koboldcpp/Noromaid-v0.4-Mixtral-Instruct-8x7b-Zloss\",\"koboldcpp/OpenHermes-2.5-Mistral-7B\",\"koboldcpp/OpenHermes-2.5-Mistral-7B\"],\"workers\":[]}")
        .header("apikey", api_key)
        .header("Origin", "https://lite.koboldai.net")
        .header("Client-Agent", "KoboldAiLite:17")
        .send()
        .await.unwrap();
    println!("{}", res.text().await.unwrap());
    Ok(())

}
