use md5::{Digest, Md5};
use rayon::prelude::*;

fn main() {
    let result = (0..1_000_000u32).into_par_iter().find_any(|n| {
        let mut candidate = "bgvyzdsv".to_string();
        candidate.push_str(&n.to_string());

        let mut hasher = Md5::new();
        hasher.update(candidate);

        let result = hasher.finalize();
        let result = base16ct::lower::encode_string(&result);

        result.as_bytes()[0..5] == [48, 48, 48, 48, 48]
    });

    println!("{:?}", result);
}
