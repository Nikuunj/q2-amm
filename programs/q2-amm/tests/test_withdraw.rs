use solana_signer::Signer;


mod utils;

use utils::*;

#[test]
fn test_deposit() {
   
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
    let max_x= 100_000_000;
    let max_y= 100_000_000;
    let deposit_ix = deposit(&payer, seed, &mint_x, &mint_y, amount_1, max_x, max_y);

    let min_x = 10_000_000;
    let min_y = 10_000_000;
    let amount_2 = 10_000_000;

    let withdraw_ix = withdraw(&payer, seed, &mint_x, &mint_y, amount_2, min_x, min_y);

    send(&mut svm, &payer, &[init_ix, deposit_ix, withdraw_ix]);
}