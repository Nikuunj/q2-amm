use anchor_lang::{
    InstructionData, ToAccountMetas, solana_program::{instruction::Instruction, pubkey::Pubkey}, system_program::ID as SYSTEM_PROGRAM_ID};
use anchor_spl::{associated_token};
use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo, TOKEN_ID};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

pub fn setup() -> (LiteSVM, Keypair) {
    let program_id = q2_amm::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/q2_amm.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    (svm, payer)
}

pub fn mint(svm: &mut LiteSVM, payer: &Keypair) -> Pubkey {
    CreateMint::new(svm, payer)
    .decimals(6)
    .authority(&payer.pubkey())
    .send()
    .unwrap()
}

pub fn get_config_pda(seed: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"config", seed.to_le_bytes().as_ref()], &q2_amm::id())
}

pub fn get_mint_lp(config_pda: &Pubkey) -> Pubkey {
    let (mint_lp, _bump) = Pubkey::find_program_address(&[b"lp", config_pda.as_ref()], &q2_amm::id());

    mint_lp
}

pub fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    associated_token::get_associated_token_address(owner, mint)
}

pub fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    CreateAssociatedTokenAccount::new(svm, payer, mint)
    .owner(owner)
    .send()
    .unwrap()
}

pub fn mint_to(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, ata: &Pubkey, amount: u64) {
    MintTo::new(svm, payer, mint, ata, amount).send().unwrap();
}

pub fn send(svm: &mut LiteSVM, payer: &Keypair, inx: &[Instruction]) {
    let msg = Message::new(inx, Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let tx = Transaction::new(&[&payer], msg, recent_blockhash);

    svm.send_transaction(tx).unwrap();

}

pub fn swap(payer: &Keypair, seed: u64, mint_x: &Pubkey, mint_y: &Pubkey, amount: u64, min: u64, is_x: bool) -> Instruction {
    let (config_pda, _config_bump) = get_config_pda(seed);

    let mint_lp = get_mint_lp(&config_pda);

    let vault_x = get_ata(&config_pda, mint_x);
    let vault_y = get_ata(&config_pda, mint_y);

    let user_x = get_ata(&payer.pubkey(), mint_x);
    let user_y = get_ata(&payer.pubkey(), mint_y);

    let swap_ix = Instruction {
        program_id: q2_amm::id(),
        accounts: q2_amm::accounts::Swap {
            user: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: config_pda,
            mint_lp,
            vault_x,
            vault_y,
            user_x,
            user_y,
            associated_token_program: associated_token::ID,
            token_program_lp: TOKEN_ID,
            token_program_x: TOKEN_ID,
            token_program_y: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID
        }.to_account_metas(None),
        data: q2_amm::instruction::Swap { is_x, amount, min }.data()
    };

    swap_ix

}

pub fn withdraw(payer: &Keypair, seed: u64, mint_x: &Pubkey, mint_y: &Pubkey, amount: u64, min_x: u64, min_y: u64) -> Instruction {
    let (config_pda, _config_bump) = get_config_pda(seed);

    let mint_lp = get_mint_lp(&config_pda);

    let vault_x = get_ata(&config_pda, mint_x);
    let vault_y = get_ata(&config_pda, mint_y);

    let user_x = get_ata(&payer.pubkey(), mint_x);
    let user_y = get_ata(&payer.pubkey(), mint_y);
    let user_lp = get_ata(&payer.pubkey(), &mint_lp);

    let withdraw_ix = Instruction {
        program_id: q2_amm::id(),
        accounts: q2_amm::accounts::Withdraw {
            user: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: config_pda,
            mint_lp,
            vault_x,
            vault_y,
            user_x,
            user_y,
            user_lp,
            associated_token_program: associated_token::ID,
            token_program_lp: TOKEN_ID,
            token_program_x: TOKEN_ID,
            token_program_y: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID
        }.to_account_metas(None),
        data: q2_amm::instruction::Withdraw { amount, min_x, min_y }.data()
    };

    withdraw_ix
}

pub fn deposit(payer: &Keypair, seed: u64, mint_x: &Pubkey, mint_y: &Pubkey, amount: u64, max_x: u64, max_y: u64) -> Instruction {
    let (config_pda, _config_bump) = get_config_pda(seed);

    let mint_lp = get_mint_lp(&config_pda);

    let vault_x = get_ata(&config_pda, mint_x);
    let vault_y = get_ata(&config_pda, mint_y);

    let user_x = get_ata(&payer.pubkey(), mint_x);
    let user_y = get_ata(&payer.pubkey(), mint_y);
    let user_lp = get_ata(&payer.pubkey(), &mint_lp);


    
    let deposit_ix = Instruction {
        program_id: q2_amm::id(),
        accounts: q2_amm::accounts::Deposit {
            user: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: config_pda,
            mint_lp,
            vault_x,
            vault_y,
            user_x,
            user_y,
            user_lp,
            associated_token_program: associated_token::ID,
            token_program_lp: TOKEN_ID,
            token_program_x: TOKEN_ID,
            token_program_y: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID
        }.to_account_metas(None),
        data: q2_amm::instruction::Deposit { amount, max_x, max_y }.data()
    };

    deposit_ix
}

pub fn initialize(payer: &Keypair, seed: u64, mint_x: &Pubkey, mint_y: &Pubkey, fee: u16) -> Instruction {
    let (config_pda, _config_bump) = get_config_pda(seed);

    let vault_x = get_ata(&config_pda, mint_x);
    let vault_y = get_ata(&config_pda, mint_y);

    let mint_lp = get_mint_lp(&config_pda);


    let init_ix = Instruction {
        program_id: q2_amm::id(),
        accounts: q2_amm::accounts::Initialize {
            initializer: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: config_pda,
            mint_lp,
            vault_x,
            vault_y,
            associated_token_program: associated_token::ID,
            token_program_lp: TOKEN_ID,
            token_program_x: TOKEN_ID,
            token_program_y: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID
        }.to_account_metas(None),
        data: q2_amm::instruction::Initialize { seed, fee, authority: Some(payer.pubkey()) }.data()
    };


    init_ix
}