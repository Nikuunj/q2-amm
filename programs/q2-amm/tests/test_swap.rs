use solana_signer::Signer;


mod utils;

use utils::*;

#[test]
fn test_swap() {
   
    let (mut svm, payer) = setup();

    let mint_x = mint(&mut svm, &payer);
    let mint_y = mint(&mut svm, &payer);

    let seed = 120;

    let init_ix = initialize(&payer, seed, &mint_x, &mint_y, 100);

    let user_x = create_ata(&mut svm, &payer, &mint_x, &payer.pubkey());
    let user_y = create_ata(&mut svm, &payer, &mint_y, &payer.pubkey());

    mint_to(&mut svm, &payer, &mint_x, &user_x, 10000_000_000);
    mint_to(&mut svm, &payer, &mint_y, &user_y, 10000_000_000);

    let amount_1 = 1000_000_000;
    let max_x= amount_1;
    let max_y= amount_1;
    let deposit_ix = deposit(&payer, seed, &mint_x, &mint_y, amount_1, max_x, max_y);


    let amount_2 = 70_000_000;
    let min = 60_000_000;
    let is_x = true;

    let swap_ix = swap(&payer, seed, &mint_x, &mint_y, amount_2, min, is_x);
    send(&mut svm, &payer, &[init_ix, deposit_ix, swap_ix]);
}