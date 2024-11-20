use rand::{rngs::StdRng, Rng, SeedableRng};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
fn main2() {
    let width = 800;
    let height = 800;

    let mut buf = image::ImageBuffer::new(width, height);

    let seed = string_to_seed("THE INPUT STRING");
    let mut rng = StdRng::from_seed(seed);
    let mut rng2 = StdRng::from_seed(seed);
    let random_value = rng.gen_range(-1.0..=1.0);
    let random_value_2 = rng2.gen_range(-1.0..=1.0);

    println!("Random value: {}", random_value);
    println!("Random value 2: {}", random_value_2);

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let scaled_x = rescale_to_minus_one_one(x as f32, 0 as f32, (width - 1) as f32);
        let scaled_y = rescale_to_minus_one_one(y as f32, 0 as f32, (height - 1) as f32);

        *pixel = compute_pixel(scaled_x, scaled_y, &more_complicated_example);
    }

    buf.save("image.png").unwrap();
}

fn main() {
    let mut rng = StdRng::from_seed(string_to_seed("THE INPUT STRING"));
    let mut mvec = Vec::new();
    for i in 0..800000 {
        mvec.push(pick_c(i as f32, i as f32, &mut rng));
    }
    let mut number_of_1 = 0;
    let mut number_of_2 = 0;
    let mut number_of_3 = 0;
    for i in mvec {
        match i {
            1 => number_of_1 += 1,
            2 => number_of_2 += 1,
            3 => number_of_3 += 1,
            _ => unreachable!(),
        }
    }
    println!("Number of 1: {}", number_of_1);
    println!("Number of 2: {}", number_of_2);
    println!("Number of 3: {}", number_of_3);
    println!("Total: {}", number_of_1 + number_of_2 + number_of_3);
}

#[allow(dead_code)]
fn example_grammar(x: f32, y: f32, rng: &mut StdRng) -> (u8, u8, u8) {
    let c = pick_c(x, y, rng);

    return (c, c, c);
}

type Branch = Vec<Rule>;

#[allow(dead_code)]
struct Rule {
    name: String,
    weight: usize,
    operation: Operation,
}

enum Operation {
    X,
    Y,
    Rule,
    Add(Box<Operation>, Box<Operation>),
    Mult(Box<Operation>, Box<Operation>),
    Random,
}

fn select_rule(branch: Branch, rng: &mut StdRng) -> Rule {
    let total_weight = branch.iter().map(|rule| rule.weight).sum();
    let random_weight = rng.gen_range(0..total_weight);
    let mut current_weight = 0;
    for rule in branch {
        current_weight += rule.weight;
        if current_weight >= random_weight {
            return rule;
        }
    }
    unreachable!()
}

fn pick_c(x: f32, y: f32, rng: &mut StdRng) -> f32 {
    let _ = x * y;
    let branch = rng.gen_range(0..8);
    let c = match branch {
        0..=1 => pick_a(x, y, rng),
        2..=4 => add(pick_c(x, y, rng), pick_c(x, y, rng)),
        5..=7 => 3,
        _ => unreachable!(),
    };
    c
}

fn pick_a(x: f32, y: f32, rng: &mut StdRng) -> f32 {
    let branch = rng.gen_range(0..3);
    let a = match branch {
        0 => rng.gen_range(-1.0..1.0),
        1 => x,
        2 => y,
        _ => unreachable!(),
    };
    a
}

fn string_to_seed(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize();

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash[..32]);
    seed
}

#[allow(dead_code)]
fn pixel_is_x(x: f32, _: f32) -> (u8, u8, u8) {
    let r = rescale_to_zero_twofiftyfive(x);
    let g = rescale_to_zero_twofiftyfive(x);
    let b = rescale_to_zero_twofiftyfive(x);
    (r, g, b)
}

#[allow(dead_code)]
fn pixel_is_y(_: f32, y: f32) -> (u8, u8, u8) {
    let r = rescale_to_zero_twofiftyfive(y);
    let g = rescale_to_zero_twofiftyfive(y);
    let b = rescale_to_zero_twofiftyfive(y);
    (r, g, b)
}

#[allow(dead_code)]
fn more_complicated_example(x: f32, y: f32) -> (u8, u8, u8) {
    if x * y > 0.0 {
        let r = rescale_to_zero_twofiftyfive(x);
        let g = rescale_to_zero_twofiftyfive(y);
        let b = 255;
        return (r, g, b);
    } else {
        let modulo = x % y;
        let r = rescale_to_zero_twofiftyfive(modulo);
        let g = rescale_to_zero_twofiftyfive(modulo);
        let b = rescale_to_zero_twofiftyfive(modulo);
        return (r, g, b);
    }
}

fn rescale_value(value: f32, min: f32, max: f32, new_min: f32, new_max: f32) -> f32 {
    (value - min) / (max - min) * (new_max - new_min) + new_min
}

fn rescale_to_minus_one_one(value: f32, min: f32, max: f32) -> f32 {
    rescale_value(value, min, max, -1 as f32, 1 as f32)
}

fn rescale_to_zero_twofiftyfive(value: f32) -> u8 {
    rescale_value(value, -1 as f32, 1 as f32, 0 as f32, 255 as f32) as u8
}

fn compute_pixel(
    scaled_x: f32,
    scaled_y: f32,
    func: &dyn Fn(f32, f32) -> (u8, u8, u8),
) -> image::Rgb<u8> {
    let (r, g, b) = func(scaled_x, scaled_y);
    image::Rgb([r, g, b])
}
