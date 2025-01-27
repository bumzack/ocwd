use generic_tools::tool_flux::flux::WhichFlux;
use generic_tools::tool_stable_diffusion::stablediff::{stable_diffusion, StableDiffusionWhich};
use generic_tools::tool_wuerstchen::wuerstchen::run_wuerstchen;
use rand::Rng;
use std::time::SystemTime;
use std::time::Instant;

fn main() {
    println!("starting, world!");

    // prompts taking from here: https://huggingface.co/docs/diffusers/v0.19.1/en/api/pipelines/kandinsky
    let prompts: Vec<(String, String)> = vec![
        ("alien".to_string(), "An alien cheeseburger creature eating itself, claymation, cinematic, moody lighting".to_string()),
        ("bird_eye_view_woman".to_string(), "bird eye view shot of a full body woman with cyan light orange magenta makeup, digital art, long braided hair her face separated by makeup in the style of yin Yang surrealism, symmetrical face, real image, contrasting tone, pastel gradient background".to_string()),
        ("car_exploding".to_string(), "A car exploding into colorful dust".to_string()),
        ("armchair".to_string(), "editorial photography of an organic, almost liquid smoke style armchair".to_string()),
        ("alien_landscape".to_string(), "birds eye view of a quilted paper style alien planet landscape, vibrant colours, Cinematic lighting".to_string()),
        ("lion".to_string(), "A lion in galaxies, spirals, nebulae, stars, smoke, iridescent, intricate detail, octane render, 8k".to_string()),
        ("fantasy_landscape".to_string(), "A fantasy landscape, Cinematic lighting".to_string()),
        ("robot_photo".to_string(), "A robot, 4k photo".to_string()),
        ("squirrel_burger".to_string(), "A painting of a squirrel eating a burger".to_string()),
        ("cat_van_gogh".to_string(), "a high resolution painting of a cat in the style of van gogh".to_string()),
        ("astronaut_mars".to_string(), "a photo of an astronaut riding a horse on mars".to_string()),
        ("horsemen_apocalypse".to_string(), "the four horsewomen of the apocalypse, painting by tom of finland, gaston bussiere, craig mullins, j. c. leyendecker".to_string()),
        ("yellow_cat".to_string(), "Face of a yellow cat, high resolution, sitting on a park bench".to_string()),
        ("two_tigers".to_string(), "two tigers".to_string()),
        ("astronaut_jungle".to_string(), "Astronaut in a jungle, cold color palette, muted colors, detailed, 8k".to_string()),
        ("majestic_tiger_bench".to_string(), "A majestic tiger sitting on a bench".to_string()),
        ("majestic_tiger_jumping".to_string(), "A majestic lion jumping from a big stone at night".to_string()),
        ("dramatic_wave".to_string(), "dramatic wave, the Oceans roar, Strong wave spiral across the oceans as the waves unfurl into roaring crests; perfect wave form; perfect wave shape; dramatic wave shape; wave shape unbelievable; wave; wave shape spectacular".to_string()),
        ("fantasy_landscape2".to_string(), "A fantasy landscape, trending on artstation".to_string()),
        ("pack_roses".to_string(), "A pack of roses".to_string()),
        ("blue_roses".to_string(), "A pack of blue roses".to_string()),
        ("capybara".to_string(), "A capybara holding a sign that reads Hello World".to_string()),
        ("hybrid_creature".to_string(), "A whimsical and creative image depicting a hybrid creature that is a mix of a waffle and a hippopotamus, basking in a river of melted butter amidst a breakfast-themed landscape. It features the distinctive, bulky body shape of a hippo. However, instead of the usual grey skin, the creature's body resembles a golden-brown, crispy waffle fresh off the griddle. The skin is textured with the familiar grid pattern of a waffle, each square filled with a glistening sheen of syrup. The environment combines the natural habitat of a hippo with elements of a breakfast table setting, a river of warm, melted butter, with oversized utensils or plates peeking out from the lush, pancake-like foliage in the background, a towering pepper mill standing in for a tree.  As the sun rises in this fantastical world, it casts a warm, buttery glow over the scene. The creature, content in its butter river, lets out a yawn. Nearby, a flock of birds take flight".to_string()),
        ("astronaut_green_horse".to_string(), "An astronaut riding a green horse".to_string()),
        ("namsan_tower".to_string(), "the Namsan Tower in korea seoul, surrounded by trees and buildings. The sky is visible in the background, and there are watermarks on the image.".to_string()),
        ("cat_hello_world".to_string(), "A cat holding a sign that says hello world".to_string()),
        ("logo_grapes".to_string(), "logo,Minimalist,A bunch of grapes and a wine glass".to_string()),
        ("handsome_gril".to_string(), "handsome girl in a suit covered with bold tattoos and holding a pistol. Animatrix illustration style, fantasy style, natural photo cinematic".to_string()),
        ("qs_cartoon_man_black_mustache".to_string(), "A cartoon drawing of a man with a black mustache and a green jacket. The mans face is pink and he is wearing a white shirt. His eyes are blue and his hair is black. The background is a vibrant red.".to_string()),
        ("qs_mans_face_purple".to_string(), "A medium-sized painting of a mans face, painted in a vibrant shade of purple. The mans eyes are a piercing blue, and he has a black beard and mustache. His eyebrows are a darker shade of blue, while his lips are a lighter shade of pink. He is wearing a long-sleeved yellow shirt with a brown stripe down the center of his chest. The background is a light peach color.".to_string()),
        ("qs_mans_face_orange".to_string(), "An eye-level painting of a mans face, painted in a vibrant orange shade, is set against a light gray background. The mans eyes are squinted, and he is wearing a dark brown suit jacket, a white collared shirt, and a black tie. His mouth is slightly open, and his lips are slightly parted, as if he is about to go to the right. His hair is dark brown, while his ears are slender and pointy.".to_string()),
        ("qs_mans_face_red_shade".to_string(), "A medium-sized painting of a mans face, painted in a vibrant shade of red. The mans eyes are a piercing green, and he has a serious look on his face. His hair is a dark brown, and his eyebrows are a lighter shade of brown. His mouth is slightly open, and there is a red line running down the middle of his face, adding a pop of color to the scene. The background is a deep blue, and the mans upper body is visible.".to_string()),
        ("qs_mans_face_orange_blue_eyes".to_string(), "A medium-sized painting of a mans face, painted in a vibrant shade of orange. The mans eyes are a piercing blue, and he is wearing a purple shirt with a white collar. His eyebrows are a darker shade of brown, and his lips are a lighter shade of red. His mouth is slightly open, and there is a cigarette dangling from his collar. The background of the painting is a deep blue.".to_string()),
        ("qw_mans_face_bright_teal_hair".to_string(), "An abstract painting of a man with spiky, bright teal hair. His face is painted in vibrant shades of orange and yellow, with bold streaks of white and black crossing his cheeks. His eyes are large and glowing white, creating a dramatic effect. He is wearing a black leather jacket adorned with metallic silver studs. The background is a chaotic mix of red, blue, and green paint splatters, adding energy to the composition.".to_string()),
        ("qw_abstract_painting_spiky".to_string(), "An abstract painting of a man with spiky, bright teal hair. His face is painted in vibrant shades of orange and yellow, with bold streaks of white and black crossing his cheeks. His eyes are large and glowing white, creating a dramatic effect. He is wearing a black leather jacket adorned with metallic silver studs. The background is a chaotic mix of red, blue, and green paint splatters, adding energy to the composition.".to_string()),
        ("qw_eye_level_depicted_orange_background".to_string(), "An eye-level painting of a mans face is depicted in a vibrant orange background. The mans eyes are a piercing blue, while his hair is a darker shade of brown. His eyebrows are a light brown, and he has a slight smile on his face. He is wearing a collared button-down shirt with a collar and a collar around his neck. His hair is cut in a bob, adding a pop of color to the scene. He is wearing a black collar with a silver chain hanging from it. His left ear is visible, and his right ear is slightly open. His eyes are slightly open, as if he is looking to the right. His mouth is slightly closed, and there are a few small white lines on the top of his head, adding depth to the image.".to_string()),
        ("qw_abstract_woman_face".to_string(), "An abstract painting of a woman with long dark brown hair. The womans face is painted in a vibrant pink shade. Her eyes are a piercing yellow. Her eyebrows are a darker shade of brown. Her lips are a lighter pink. She is wearing a black dress with a silver necklace around her neck. Her earrings are dangling from her ears. The background is a vibrant red with yellow and blue lines.".to_string()),
        ("qw_man_vibrant_ping_background".to_string(), "An eye-level sketch of a mans face is depicted in a vibrant pink background. The mans eyes are squinted, and he has a mustache and goatee. His hair is a dark shade of purple, and his eyebrows are a lighter shade of brown. His mouth is slightly open, with a smile on its face. He is wearing a collared button-down shirt with a collar and long sleeves. The shirt is adorned with black lines, adding a pop of color to the scene".to_string()),
        ("qw_young_man_curly_dark_brown".to_string(), "An eye-level sketch of a young man with curly dark brown hair and glasses. His eyes are a calm gray, and he has a soft smirk on his lips. His face is framed by a navy-blue hoodie, slightly pulled over his head, and a silver pendant is visible around his neck. The background is a misty gray with faint white lines radiating from the figure, suggesting a subtle glow.".to_string()),
        ("chocolate_cookie".to_string(), "A chocolate cookie".to_string()),
        ("blue_paradise_jungle".to_string(), "a blue paradise bird in the jungle".to_string()),
        ("grungy_wolam_rainbow_hair".to_string(), "a grungy woman with rainbow hair, travelling between dimensions, dynamic pose, happy, soft eyes and narrow chin, extreme bokeh, dainty figure, long hair straight down, torn kawaii shirt and baggy jeans, In style of by Jordan Grimmer and greg rutkowski, crisp lines and color, complex background, particles, lines, wind, concept art, sharp focus, vivid colors".to_string()),
        ("rocket_ship_space".to_string(), "rocket ship in deep space".to_string()),
        ("lone_soldier_post_apocalyptic".to_string(), "lone soldier in a post apocalyptic city street".to_string()),
    ];

    // https://huggingface.co/dreamlike-art/dreamlike-diffusion-1.0
    //  "dreamlikeart, a grungy woman with rainbow hair, travelling between dimensions, dynamic pose, happy, soft eyes and narrow chin, extreme bokeh, dainty figure, long hair straight down, torn kawaii shirt and baggy jeans, In style of by Jordan Grimmer and greg rutkowski, crisp lines and color, complex background, particles, lines, wind, concept art, sharp focus, vivid colors"

    // https://replicate.com/adithram/inkpunk-diffusion
    //  "a photo of an astronaut riding a horse on mars nvinkpunk".to_string()
    //  "rocket ship in deep space nvinkpunk".to_string()
    //  "lone soldier in a post apocalyptic city street nvinkpunk".to_string()

    // "https://civitai.com/images/2309064"

    // https://civitai.com/images/26604316
    // "nvinkpunk, a (DONATEYOURLIKENESS|woman), beautiful, (long hair:1.2), intricate details, modelshoot style, dreamlikeart, dramatic lighting, 8k, highly detailed, trending artstation".to_string()

    // https://civitai.com/images/20836362
    // "1girl, nvinkpunk, masterpiece, best quality, award winning, High Detail ,8k, intricate, colors, random dripping paint, splashes, beautiful scenery, mountain, forest, castle,  samus aran, zero suit".to_string()

    // "Qs Sketch, A cartoon drawing of a man with a black mustache and a green jacket. The mans face is pink and he is wearing a white shirt. His eyes are blue and his hair is black. The background is a vibrant red.".to_string()
    // "Qs Sketch, A medium-sized painting of a mans face, painted in a vibrant shade of purple. The mans eyes are a piercing blue, and he has a black beard and mustache. His eyebrows are a darker shade of blue, while his lips are a lighter shade of pink. He is wearing a long-sleeved yellow shirt with a brown stripe down the center of his chest. The background is a light peach color.".to_string()
    //  "Qs Sketch, An eye-level painting of a mans face, painted in a vibrant orange shade, is set against a light gray background. The mans eyes are squinted, and he is wearing a dark brown suit jacket, a white collared shirt, and a black tie. His mouth is slightly open, and his lips are slightly parted, as if he is about to go to the right. His hair is dark brown, while his ears are slender and pointy.".to_string()
    // "Qs Sketch, A medium-sized painting of a mans face, painted in a vibrant shade of red. The mans eyes are a piercing green, and he has a serious look on his face. His hair is a dark brown, and his eyebrows are a lighter shade of brown. His mouth is slightly open, and there is a red line running down the middle of his face, adding a pop of color to the scene. The background is a deep blue, and the mans upper body is visible.".to_string()
    // "Qs Sketch, A medium-sized painting of a mans face, painted in a vibrant shade of orange. The mans eyes are a piercing blue, and he is wearing a purple shirt with a white collar. His eyebrows are a darker shade of brown, and his lips are a lighter shade of red. His mouth is slightly open, and there is a cigarette dangling from his collar. The background of the painting is a deep blue.".to_string()
    // "Qs Sketch, An abstract painting of a mans face, painted in a vibrant shade of pink. The mans eyes are yellow, with black eyebrows, and a black nose. He is wearing a black suit jacket, a white collared shirt and a blue tie. The background of the painting is a vibrant orange, creating a striking contrast to the painting.".to_string()

    // https://huggingface.co/strangerzonehf/Qw-Sketch

    //   "Qw-Sketch, An abstract painting of a man with spiky, bright teal hair. His face is painted in vibrant shades of orange and yellow, with bold streaks of white and black crossing his cheeks. His eyes are large and glowing white, creating a dramatic effect. He is wearing a black leather jacket adorned with metallic silver studs. The background is a chaotic mix of red, blue, and green paint splatters, adding energy to the composition.".to_string()
    //   "Qw-Sketch, An eye-level painting of a mans face is depicted in a vibrant orange background. The mans eyes are a piercing blue, while his hair is a darker shade of brown. His eyebrows are a light brown, and he has a slight smile on his face. He is wearing a collared button-down shirt with a collar and a collar around his neck. His hair is cut in a bob, adding a pop of color to the scene. He is wearing a black collar with a silver chain hanging from it. His left ear is visible, and his right ear is slightly open. His eyes are slightly open, as if he is looking to the right. His mouth is slightly closed, and there are a few small white lines on the top of his head, adding depth to the image.".to_string()
    //  "Qw-Sketch, An abstract painting of a woman with long dark brown hair. The womans face is painted in a vibrant pink shade. Her eyes are a piercing yellow. Her eyebrows are a darker shade of brown. Her lips are a lighter pink. She is wearing a black dress with a silver necklace around her neck. Her earrings are dangling from her ears. The background is a vibrant red with yellow and blue lines.".to_string()
    //  "Qw-Sketch, An eye-level sketch of a mans face is depicted in a vibrant pink background. The mans eyes are squinted, and he has a mustache and goatee. His hair is a dark shade of purple, and his eyebrows are a lighter shade of brown. His mouth is slightly open, with a smile on its face. He is wearing a collared button-down shirt with a collar and long sleeves. The shirt is adorned with black lines, adding a pop of color to the scene".to_string()
    //  "Qw-Sketch, An eye-level sketch of a young man with curly dark brown hair and glasses. His eyes are a calm gray, and he has a soft smirk on his lips. His face is framed by a navy-blue hoodie, slightly pulled over his head, and a silver pendant is visible around his neck. The background is a misty gray with faint white lines radiating from the figure, suggesting a subtle glow.".to_string()

    let prompts = prompts.iter().take(1).cloned().collect();

    run_flux(&prompts);
    run_stable_diffusion(&prompts);
    run_wwwwuerstchen(&prompts);
}

fn run_flux(prompts: &Vec<(String, String)>) {
    for (prompt, ffilename) in prompts {
        let seed: u64 = rand::rng().random_range(0..u32::MAX - 10) as u64;
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("flux_schnell_{}_{}", &ffilename, ts);

        let start = Instant::now();
        let res = generic_tools::tool_flux::flux::run_flux(
            prompt.to_string(),
            true,
            1280,
            720,
            "./".to_string(),
            filename.clone(),
            WhichFlux::Schnell,
            seed,
        );

        if res.is_err() {
            println!("error creating image {}", filename);
        } else {
            println!("success creating image {}", filename);
        }
        let duration = start.elapsed();
        println!("flux_schnell finished after {}secs", duration.as_secs());

        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("flux_dev_{}_{}", &ffilename, ts);

        let start = Instant::now();
        let seed: u64 = rand::rng().random_range(0..u32::MAX - 10) as u64;
        let res = generic_tools::tool_flux::flux::run_flux(
            prompt.to_string(),
            true,
            1280,
            720,
            "./".to_string(),
            filename.clone(),
            WhichFlux::Dev,
            seed,
        );
        if res.is_err() {
            println!("error creating image {}", filename);
        } else {
            println!("success creating image {}", filename);
        }
        let duration = start.elapsed();
        println!("flux_dev finished after {}secs", duration.as_secs());
    }
}

fn run_stable_diffusion(prompts: &Vec<(String, String)>) {
    for (prompt, ffilename) in prompts {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("stable_diffusion_{}_{}", &ffilename, ts);
        let start = Instant::now();

        let res = stable_diffusion(
            prompt.to_string(),
            filename.clone(),
            "jpg".to_string(),
            StableDiffusionWhich::V3_5Large,
        );
        if res.is_err() {
            println!("error creating image {}", filename);
        } else {
            println!("success creating image {}", filename);
        }

        let duration = start.elapsed();
        println!("stable_diffusion finished after {}secs", duration.as_secs());
    }
}

fn run_wwwwuerstchen(prompts: &Vec<(String, String)>) {
    for (prompt, ffilename) in prompts {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = format!("wuerstchen_{}_{}", &ffilename, ts);

        let start = Instant::now();

        let res = run_wuerstchen(prompt.to_string(), filename.clone());
        if res.is_err() {
            println!("error creating image {}", filename);
        } else {
            println!("success creating image {}", filename);
        }
        let duration = start.elapsed();

        println!("wuerstchen finished after {}secs", duration.as_secs());
    }
}
