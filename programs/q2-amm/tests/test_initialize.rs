mod utils;

use utils::*;

#[test]
fn test_initialize() {
    let (mut svm, payer) = setup();

    let mint_x = mint(&mut svm, &payer);
    let mint_y = mint(&mut svm, &payer);

    let seed = 120;

    let init_ix = initialize(&payer, seed, &mint_x, &mint_y, 100);

    send(&mut svm, &payer, &[init_ix]);
}
