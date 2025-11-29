// encryption utilities

#[derive(Debug)]
pub enum KdfMode {
    // Encryption Mode (or read only mode): We typically require it to be 
    // faster since we derive the key every time we verify a register's 
    // existence or match it with others. Processing times of 600-900ms
    // for such basic operations seem nonsensical.
    //
    EncrM, 

    // For decryption data or matching the key provided by the user from its
    // reg_name, we intend it to be slow to prevent fast brute-force operations
    //
    DecryM,
}
