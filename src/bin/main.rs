use mixnet_rust::{el_gamal::ElGamal, shuffler::Shuffler, utils::*, verifier::Verifier, N};
use rand::random_range;

fn main() {
    let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    let mut g = random_range(2..p-1);
    g = modexp(g, 2, p);

    // println!("let (p, q, g) = ({p}, {q}, {g});");
    // let (p, q, g) = (2109266063, 1054633031, 658647440);

    // test
    // let (p, q, g) = (2179524563, 1089762281, 219491794);

    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let mut h_list: [u32; N] = [0; N];
    for i in 0..N {
        let mut h = random_range(2..p-1);
        h = modexp(h, 2, p);
        h_list[i] = h;
    }

    // println!("h_list = {:?}", h_list);
    // h_list = [1536194482, 2107128547, 711987449];

    // test
    // let h_list: [u32; N] = core::array::from_fn(|i| modexp(i as u32 + 1, 4, p));

    let shuffler = Shuffler::new(p, q, g, h_list, el_gamal.pk());

    let plaintext_list: [u32; N] = core::array::from_fn(|i| modexp(i as u32 + 1, 2, p));
    let ciphertext_list_1: [(u32, u32); N] = core::array::from_fn(|i| el_gamal.encrypt(plaintext_list[i]));

    // println!("let ciphertext_list_1 = {:?};", ciphertext_list_1);
    // let ciphertext_list_1 = [(236933124, 370178551), (1449921388, 323335557), (1118392396, 1680580567)];
    
    println!("plaintext: {:?}", plaintext_list);
    println!("ciphertext: {:?}", ciphertext_list_1);

    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(ciphertext_list_1);
    let proof = shuffler.gen_proof(
        ciphertext_list_1,
        ciphertext_list_2,
        random_list,
        psi
    );
    println!("shuffled: {:?}", ciphertext_list_2);
    println!("proof: {:?}", proof);

    let verifier = Verifier::new(p, q, g, h_list);
    let result = verifier.check_proof(proof, ciphertext_list_1, ciphertext_list_2, el_gamal.pk());
    println!("result: {result}");

    let decrypted_list: [u32; N] = core::array::from_fn(|i| el_gamal.decrypt(ciphertext_list_2[i]));
    println!("shuffled & decrypted: {:?}", decrypted_list);
}
