//! reference implementation for BIP-44 wallets

// Seed mnemonics should be interchangeable between wallets which use BIP-44.
// This should be the default key generation scheme for Libra wallets.
// There is a legacy version which implemented Diem-wallet. That key generation scheme was not par of any BIP standard. It should be deprecated. Legacy account menmonics will still work with the BIP-44 implementation. But the private keys will be different. Users should rotate their Authentication Key on-chain to use the private keys generated by the BIP-44 implementation. That way the user will keep their address, and mnemonic string. But the private keys will be different. This will allow the user to use the same mnemonic string with different wallets
