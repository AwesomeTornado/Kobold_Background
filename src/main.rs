use std::{env, };


fn main() {
    println!("Hello, Kobold_background developers!");

    let key_name = "Kobold_BG_api_Key";
    let mut api_key = "";
    let key_set = env::var(key_name).is_err();

    if(!key_set){
        api_key = &*env::var(key_name).expect("0000000000")
    }else {
        std::println!("Can't find your API key in any env variables, Please set the path variable \"Kobold_BG_api_Key\" to your koboldai api key\n Thanks");
        panic!("Path variable not set.");
    }

    //if(env::var(key_name))
    //let mut apiKey = String::new();
    //env::set_var(key, "0000000000");
}
