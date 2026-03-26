use e2easy_pc::{e2easy::E2Easy, io_helpers::{read_json, request_user_input, write_json}, types::*};

const CHALLENGE: &str = "2";
// const CAST: &str = "1";
fn main () {
    println!("\n\n--------- Iniciando urna eletronica... ------------");

    let election_config: ElectionConfig = read_json("./config/election_config.json").unwrap();
    let (h, h_list_seed) = (election_config.crypto.h, election_config.crypto.h_list_seed);
    
    let mut e2easy = E2Easy::new(&h, h_list_seed, None);

    loop {
        let is_new_voter = request_user_input("\nReceber novo voto? (s/n): ").unwrap();
        if is_new_voter == "n" {
            break;
        }

        let vote1 = request_user_input("Por favor, digite seu voto para presidente: ").unwrap()
            .parse::<u32>().unwrap();
        println!("Voto confirmado: {}", vote1);

        let vote2 = request_user_input("Por favor, digite seu voto para governador: ").unwrap()
            .parse::<u32>().unwrap();
        println!("Voto confirmado: {}", vote2);

        let votes = vec![Vote{ contest: 0, choice: vote1 }, Vote{ contest: 1, choice: vote2 }];

        let (tracking_code, timestamp) = e2easy.vote(votes);
        println!("\nObrigado. Aqui está o seu código de rastreio: {}", serde_json::to_string(&tracking_code).unwrap());

        let challenge_or_cast = request_user_input("Deseja (1) lançar o voto ou (2) desafiar a urna? ").unwrap();
        if challenge_or_cast == CHALLENGE {
            let (previous_hash, _votes, nonce) = e2easy.challenge();

            println!("Aqui estão os dados para o desafio:");
            println!("    hash anterior: {}", serde_json::to_string(&previous_hash).unwrap());
            println!("    nonce: {}", serde_json::to_string(&nonce).unwrap());
            println!("    carimbo de tempo: {}", timestamp);
            println!("Voto descartado. Vote novamente.")
        } else {
            let tc_signature = e2easy.cast();
            println!("Aqui está a assinatura do seu código de rastreio:\n{}", serde_json::to_string(&tc_signature).unwrap());
            println!("Voto lançado! Obrigado por votar.");
        }
    }

    println!("\nEncerrando urna eletrônica...");
    println!("Misturando os votos e gerando as provas...");

    let (rdv_prime, rdcv, rdcv_prime, zkp_output) = e2easy.tally();

    write_json(&rdv_prime,  "./outputs/rdv_prime.json",  Some(&e2easy.sign(&rdv_prime))).unwrap();
    write_json(&rdcv,       "./outputs/rdcv.json",        Some(&e2easy.sign(&rdcv))).unwrap();
    write_json(&rdcv_prime, "./outputs/rdcv_prime.json",  Some(&e2easy.sign(&rdcv_prime))).unwrap();
    write_json(&zkp_output, "./outputs/zkp_output.json",  Some(&e2easy.sign(&zkp_output))).unwrap();
    
    println!("Arquivos criados em /outputs/");
    println!("--------- Urna eletrônica encerrada ------------\n\n");
}