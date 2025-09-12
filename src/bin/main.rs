use mixnet_rust::{el_gamal::ElGamal, shuffler::Shuffler, utils::*, verifier::Verifier, Number, ModNumber, ModNumberParams, Ciphertext, N};

fn main() {
    let p = ModNumberParams::new_vartime(Number::from_be_hex("BF4AAA250D7578E410D0DC2D68645146113D1CE9D9DD2D522BF403BF41405613").to_odd().unwrap());
    let q = Number::from_be_hex("5FA5551286BABC7208686E16B43228A3089E8E74ECEE96A915FA01DFA0A02B09").to_nz().unwrap();
    /*
    let (p, q) = safe_prime().unwrap();
    */
    let g = get_generator(&p).unwrap();
    let p_num = p.modulus().get();
    let g_num = g.retrieve();

    // println!("let (p, q, g) = ({p_num}, {q}, {g_num});");
    // let (p, q, g) = (2109266063, 1054633031, 658647440);

    // test
    // let (p, q, g) = (2179524563, 1089762281, 219491794);
    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let mut h_list: [ModNumber; N] = [ModNumber::zero(*g.params()); N];
    for i in 0..N {
        h_list[i] = get_generator(&p).unwrap();
    }

    // println!("h_list = {:?}", h_list);
    // h_list = [1536194482, 2107128547, 711987449];

    // test
    // let h_list: [u32; N] = core::array::from_fn(|i| modexp(i as u32 + 1, 4, p));

    let shuffler = Shuffler::new(p, q, g, h_list, el_gamal.pk());

    let plaintext_list: [Number; N] = core::array::from_fn(|i| ModNumber::new(&Number::from_u32(i as u32 + 1), p).square().retrieve());
    let ciphertext_list_1: [Ciphertext; N] = core::array::from_fn(|i| el_gamal.encrypt(plaintext_list[i]));

    // println!("let ciphertext_list_1 = {:?};", ciphertext_list_1);
    // let ciphertext_list_1 = [(236933124, 370178551), (1449921388, 323335557), (1118392396, 1680580567)];
    
    // println!("plaintext: {:?}", plaintext_list);
    // println!("ciphertext: {:?}", ciphertext_list_1);
  
    let (ciphertext_list_2, random_list, psi) = shuffler.gen_shuffle(ciphertext_list_1);
    let proof = shuffler.gen_proof(ciphertext_list_1, ciphertext_list_2, random_list, psi);
    // println!("shuffled: {:?}", ciphertext_list_2);
    // println!("proof: {:?}", proof);
  
    let verifier = Verifier::new(p, q, g, h_list);
    let result = verifier.check_proof(proof, ciphertext_list_1, ciphertext_list_2, el_gamal.pk());
    println!("result: {result}");

    let decrypted_list: [Number; N] = core::array::from_fn(|i| el_gamal.decrypt(ciphertext_list_2[i]));
    // println!("shuffled & decrypted: {:?}", decrypted_list);
}
