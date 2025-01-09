use crate::mistral_nemo_12b::mistral_nemo_12b;
use crate::mistral_small_latest::mistral_small_latest;

mod deepseek;
mod mistral_small_latest;
mod qwen25coder;
mod qwq_latest;
mod mistral_nemo_12b;

fn main() {
    // qwen25coder().expect("qwen25coder");

    // mistral_small_latest();
    mistral_nemo_12b();
}
