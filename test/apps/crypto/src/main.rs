use pkcs11;
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn test_aes(){
    let key = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([42u8; 16]);

    // Initialize cipher
    let cipher = Aes128::new(&key);

    let block_copy = block.clone();

    // Encrypt block in-place
    cipher.encrypt_block(&mut block);

    // And decrypt it back
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);
}

fn main() {
    println!("====== crypto user process ======");
    test_aes();
}
