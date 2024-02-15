//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let start = std::time::Instant::now();
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    // Read storage key
    let storage_key =
        "0x0000000000000000000000000000000000000000000000000000000000000002".to_string();
    // Read storage value
    let storage_value =
        "30000000000050000001173585821419940366555131416710628997561811671104543".to_string();
    // read siblings bytes encoded
    let siblings = "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000540000000000000000000000000000000000000000000000000000000000000078000000000000000000000000000000000000000000000000000000000000009a00000000000000000000000000000000000000000000000000000000000000a800000000000000000000000000000000000000000000000000000000000000214f90211a07c212f89907d2a0db43ce961cba30baca2d7dbb6a148211a21ab9d0e28053399a011bddc9f2225ed0c582092992043e6f110476c0f2f987ae1635c775d953ea73da07d1af10d326106905d0ef2f714f81250cddc5c677e84a5b861f46a885010c3fda04bc34d26a0635653bc10cb9b9c72894ec5665b1ea2617827801e1c939c9f3acca07d3f3904d9c07b1b29297ff2d1f36e62500011be4c9f28283670a88f4363af28a052259cfb8fdca4e9125d87001c69c428db5fb056d204989c4cd375db39d07159a081a06dfd00076684976b3ab4d146ba84e7113002daa354422c72aae00804db9fa055cb6cba5058ed019aa505c94f13d055671751dc3b8cf17fe30eea52949c4faaa019c2833ce2fcf336d14d38e4fb7f4aa2c8416bb5d3059ec0afc832b1afc87dada074e25a876105d13b10dc3c0dec9ac310d9a562a26059d6dca3e91139a3fd50f0a0d9f63f833f4da302f36420f370608a63fb334bb7470ce37a52edb0b2126445cba032bad7d614105aa83d0c4fbd3dda107511a9f64913194430ea26fef0aae3c0d0a0aa5cfcc1649cc912470ed938ed0715eea48961d6fdd0ea9fd07b2d01ac0cffbba08fdf2d2a0799ed9495805c968961e4c0f6257cddac725034e8cc290e29cc5250a03b98eb426d8bd194a230fae3f63dac1546dae27a4b50918e7763d21cd650fc7fa0a5a0e119eecf5c1b2312cf13e43bf87f83f12efefe098367b46355e669010e13800000000000000000000000000000000000000000000000000000000000000000000000000000000000000214f90211a0bbee5ffd7e4eaf7c6cf39fbb9ab4e301dc77d0a9ca6a29dfe29f6b8968dcd7a1a042cb34fce18bee606a6458b5bbdc2eb355d4016bd920f29256093d620129f784a0b20ad15188b8c6a651f5deddf19313f50c19e03a76c107b46b5d313ea000fb06a005cbd462796ef5c2e6f7383eb58b4612371d5e2561ce7789a4cedc20292f2d3aa08831ce5a38b0d00faaf3b503527c455d2778a7f7184fb4c0d7ef51b89fd6db84a05e4c919c950c371c511232d9e91f73c320b61bcbdaf8a17d2af4e4f0dc11ff01a0f3db4d86fc450643dc9816fc15abefb70ce70ec7a2ba1d552829c5cc617198b7a0f4b7433a3757efff1a87c141024c64b2c2d0445f5c90152bdbf904eea7b3ae5ca0bbe896ab04bf2c6911b6c17c484dc8aa8c3324426c1f8f64fce9473b5aebb46ba02f02d2d5707298b651c08af2186b5e81ac7bdccf05c349c74fdbd48ade91e5cba0ce1ae72f7b5d15cbb57cb01dc37d4d3095e9781dcdb7f9148e3f8f3b6956dc12a01a2868e90139e06ee84af2f16bb5f71e3b0104f33b9c7abf913714977dc37cffa08290f13f9e34a188333812b28df0b5c945552691da2c37318fcafb28494bdd8fa04c11e51629dd59f49e788c1190ea213c0cd04110994241b08ff70af70f3a7869a02f8403fd221714385623cb4acaf5ad00e738693388abe9e57769d2bb7b37c114a0fbab06923c73b5f96464911b34d24382a2e58f13c70728265faa8cd5457031ef800000000000000000000000000000000000000000000000000000000000000000000000000000000000000214f90211a025fc8f4362142869a521337fd9adefecaaddb6e0b2eee4030a584cd71c28941ba0ed816fbc1f8e4ec0b5b0b3931cb85d34c8a9a3ec15609704a4726d3dd0524daea0a1193dad3a824302aa4357fa8ba7c7a2ae73e5532737a3b1230f3ce7b8bf6880a03842234eb8948e82b046d1b15c429631658409016f60513fb214bc58a21f19d7a088a96e51017cc6027444c8febea637c5aee0f3ad9e6f07fb6f76030f29cd825ba0c01ef3bdac9c190ef7d9dd000a55f066005bfcdf01ef728a96c2ac1674504b78a0c2961c9c927183115a1ba03863c8d8d1a5a4a629f0b51bfd77cff144124b5a78a042d4cb2ece2a044286c7054b47df65b905988b2bcf82f56fcd3381144b8ec30ca0a1a5fe280505bed8127c1e13c5209c6b33a95e7200e55a2e7240cf9c7fe685b1a0153731307ca6d63be2943c51358fefa69817a4bd90e08f47409502d5bb08d30ba04eaa820b6a8fd17c10b49066c16f3cd2288c8a71f51f3a09e50788282815a137a0afbdb5a10fe607462ddbe57a398608ae7a579eb54b8922834e90158382831514a0959ead1bbb0c50e5f17edccd0870c5db1f7df497e0f80d87c415412d07d6689ba088ebce032a7fe21a162e73052fda0a840a5e1cea9b8ea84be64600ad8454fea1a04c680ede8b6f032a336d66aa3919e72d9bf1747e09ffad1b1c071830900ac962a0c420ef10611a7cbecd4e2036528d095600a7018439b25036936f6f837b2dee4a8000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f4f901f1a08481886000e1eade3b0426a6b2c83f69b234856bba04e93c42db0aad9ee2fafda0a3ab5cb0346dcb6fd2dd9ff811f0fd6d443944707520c9c591d39d2ea23e1e0ba0d9f48202652958a0b65c3472ad05d061742c6fcfb296fea6ce6320aefc0068c3a0cbb50be946373e8860e94770e40abc0c269ad0faa3ee4623520daf5c87800fdaa084cbc9e0f5cd6130be04fada4e6775eb7249ea0b708287df7cc8373fbdc3fdeda031c203c6e2dc10585b51f7fdc6262df28953729ecb7d2021f92609f24f0514b9a05185f9055ff81db26cff65cec970094335fbe6d165dd01cdf2432a38e0a033a6a0d75138fa12429e89a6eaf26067582aae79b47ed80b7f944bc88b4d93c24bd2e8a0ff6448f40ae03dead80c0db0e3b06f84e3695e5cd5cf28b294e6308ae91585aaa08550adab0c199f3827e303ebc89b772b6306b3ea53658b83160b5049f129cf35a0043f1931f13618f8803e98f5b5e8a8093b1647e0da1bcfb0ede3ca083262368fa02b3235166140c471331995ed4f2a896600accbf8dafbd4e22aba6347904b168780a01da9c86ab437c2bedf27a616b4dbd2332dddf8bc05eef6306a1c936dd0686177a0ad56eeb8ba02a41e1b8740520275a909e7b2f54367eccbf4884fcc90ce9aeed0a0c9ff282d9d3ec85f854cde357d2bbc65d1e400a49a3dce3b6c7fe596425d6e448000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b3f8b18080808080a0348122832f753aead522565162c3bfb3a2ad7fe477bbe2f8ca2696ae35a64f4a8080a0f6c97e9df1e36f10ad1ccfb6deea2a4098a03606332303b755a0d5898151623c80808080a058422ea358cec3f51a60941d504677f94a34a6d5d78623f5d4908085a133fec3a0fac8129af83a04a89a5094dd668fc4a7cfbfb4c01e380892db39ef235fc68b54a0dc9adf2801a8f5bbdc68206741da034ef19078685246daac910603d7db652b0480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041f83f9e37fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace9f9e0458c308499d5281a6db800b91d733d70f8ac2ca87a613643b61db36bc1f00000000000000000000000000000000000000000000000000000000000000".to_string();
    // read root
    let root = "0xedd39c02c3e79949deea0f3c06fbc7fd73ad5d01327fa60c9d798b310fe601fe".to_string();
    stdin.write(&storage_key);
    stdin.write(&storage_value);
    stdin.write(&siblings);
    stdin.write(&root);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    let end = std::time::Instant::now();
    println!("Proof generation time: {:?}", end.duration_since(start));

    // Read output.
    let value = proof.stdout.read::<String>();

    println!("valid storage value: {}", value);

    let start = std::time::Instant::now();
    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");
    let end = std::time::Instant::now();
    println!("Verification time: {:?}", end.duration_since(start));

    println!("succesfully generated and verified proof for the program!")
}

//? this is how bytes[] is encoded
// fn encode_proofs() {
//     let path1_string = "0xf90211a07c212f89907d2a0db43ce961cba30baca2d7dbb6a148211a21ab9d0e28053399a011bddc9f2225ed0c582092992043e6f110476c0f2f987ae1635c775d953ea73da07d1af10d326106905d0ef2f714f81250cddc5c677e84a5b861f46a885010c3fda04bc34d26a0635653bc10cb9b9c72894ec5665b1ea2617827801e1c939c9f3acca07d3f3904d9c07b1b29297ff2d1f36e62500011be4c9f28283670a88f4363af28a052259cfb8fdca4e9125d87001c69c428db5fb056d204989c4cd375db39d07159a081a06dfd00076684976b3ab4d146ba84e7113002daa354422c72aae00804db9fa055cb6cba5058ed019aa505c94f13d055671751dc3b8cf17fe30eea52949c4faaa019c2833ce2fcf336d14d38e4fb7f4aa2c8416bb5d3059ec0afc832b1afc87dada074e25a876105d13b10dc3c0dec9ac310d9a562a26059d6dca3e91139a3fd50f0a0d9f63f833f4da302f36420f370608a63fb334bb7470ce37a52edb0b2126445cba032bad7d614105aa83d0c4fbd3dda107511a9f64913194430ea26fef0aae3c0d0a0aa5cfcc1649cc912470ed938ed0715eea48961d6fdd0ea9fd07b2d01ac0cffbba08fdf2d2a0799ed9495805c968961e4c0f6257cddac725034e8cc290e29cc5250a03b98eb426d8bd194a230fae3f63dac1546dae27a4b50918e7763d21cd650fc7fa0a5a0e119eecf5c1b2312cf13e43bf87f83f12efefe098367b46355e669010e1380";
//     let path1_bytes = Vec::from_hex(path1_string).expect("Invalid hex string");
//     let path1 = DynSolValue::Bytes(path1_bytes);

//     let path2_string = "0xf90211a0bbee5ffd7e4eaf7c6cf39fbb9ab4e301dc77d0a9ca6a29dfe29f6b8968dcd7a1a042cb34fce18bee606a6458b5bbdc2eb355d4016bd920f29256093d620129f784a0b20ad15188b8c6a651f5deddf19313f50c19e03a76c107b46b5d313ea000fb06a005cbd462796ef5c2e6f7383eb58b4612371d5e2561ce7789a4cedc20292f2d3aa08831ce5a38b0d00faaf3b503527c455d2778a7f7184fb4c0d7ef51b89fd6db84a05e4c919c950c371c511232d9e91f73c320b61bcbdaf8a17d2af4e4f0dc11ff01a0f3db4d86fc450643dc9816fc15abefb70ce70ec7a2ba1d552829c5cc617198b7a0f4b7433a3757efff1a87c141024c64b2c2d0445f5c90152bdbf904eea7b3ae5ca0bbe896ab04bf2c6911b6c17c484dc8aa8c3324426c1f8f64fce9473b5aebb46ba02f02d2d5707298b651c08af2186b5e81ac7bdccf05c349c74fdbd48ade91e5cba0ce1ae72f7b5d15cbb57cb01dc37d4d3095e9781dcdb7f9148e3f8f3b6956dc12a01a2868e90139e06ee84af2f16bb5f71e3b0104f33b9c7abf913714977dc37cffa08290f13f9e34a188333812b28df0b5c945552691da2c37318fcafb28494bdd8fa04c11e51629dd59f49e788c1190ea213c0cd04110994241b08ff70af70f3a7869a02f8403fd221714385623cb4acaf5ad00e738693388abe9e57769d2bb7b37c114a0fbab06923c73b5f96464911b34d24382a2e58f13c70728265faa8cd5457031ef80";
//     let path2_bytes = Vec::from_hex(path2_string).expect("Invalid hex string");
//     let path2 = DynSolValue::Bytes(path2_bytes);

//     let path3_string = "0xf90211a025fc8f4362142869a521337fd9adefecaaddb6e0b2eee4030a584cd71c28941ba0ed816fbc1f8e4ec0b5b0b3931cb85d34c8a9a3ec15609704a4726d3dd0524daea0a1193dad3a824302aa4357fa8ba7c7a2ae73e5532737a3b1230f3ce7b8bf6880a03842234eb8948e82b046d1b15c429631658409016f60513fb214bc58a21f19d7a088a96e51017cc6027444c8febea637c5aee0f3ad9e6f07fb6f76030f29cd825ba0c01ef3bdac9c190ef7d9dd000a55f066005bfcdf01ef728a96c2ac1674504b78a0c2961c9c927183115a1ba03863c8d8d1a5a4a629f0b51bfd77cff144124b5a78a042d4cb2ece2a044286c7054b47df65b905988b2bcf82f56fcd3381144b8ec30ca0a1a5fe280505bed8127c1e13c5209c6b33a95e7200e55a2e7240cf9c7fe685b1a0153731307ca6d63be2943c51358fefa69817a4bd90e08f47409502d5bb08d30ba04eaa820b6a8fd17c10b49066c16f3cd2288c8a71f51f3a09e50788282815a137a0afbdb5a10fe607462ddbe57a398608ae7a579eb54b8922834e90158382831514a0959ead1bbb0c50e5f17edccd0870c5db1f7df497e0f80d87c415412d07d6689ba088ebce032a7fe21a162e73052fda0a840a5e1cea9b8ea84be64600ad8454fea1a04c680ede8b6f032a336d66aa3919e72d9bf1747e09ffad1b1c071830900ac962a0c420ef10611a7cbecd4e2036528d095600a7018439b25036936f6f837b2dee4a80";
//     let path3_bytes = Vec::from_hex(path3_string).expect("Invalid hex string");
//     let path3 = DynSolValue::Bytes(path3_bytes);

//     let path4_string ="0xf901f1a08481886000e1eade3b0426a6b2c83f69b234856bba04e93c42db0aad9ee2fafda0a3ab5cb0346dcb6fd2dd9ff811f0fd6d443944707520c9c591d39d2ea23e1e0ba0d9f48202652958a0b65c3472ad05d061742c6fcfb296fea6ce6320aefc0068c3a0cbb50be946373e8860e94770e40abc0c269ad0faa3ee4623520daf5c87800fdaa084cbc9e0f5cd6130be04fada4e6775eb7249ea0b708287df7cc8373fbdc3fdeda031c203c6e2dc10585b51f7fdc6262df28953729ecb7d2021f92609f24f0514b9a05185f9055ff81db26cff65cec970094335fbe6d165dd01cdf2432a38e0a033a6a0d75138fa12429e89a6eaf26067582aae79b47ed80b7f944bc88b4d93c24bd2e8a0ff6448f40ae03dead80c0db0e3b06f84e3695e5cd5cf28b294e6308ae91585aaa08550adab0c199f3827e303ebc89b772b6306b3ea53658b83160b5049f129cf35a0043f1931f13618f8803e98f5b5e8a8093b1647e0da1bcfb0ede3ca083262368fa02b3235166140c471331995ed4f2a896600accbf8dafbd4e22aba6347904b168780a01da9c86ab437c2bedf27a616b4dbd2332dddf8bc05eef6306a1c936dd0686177a0ad56eeb8ba02a41e1b8740520275a909e7b2f54367eccbf4884fcc90ce9aeed0a0c9ff282d9d3ec85f854cde357d2bbc65d1e400a49a3dce3b6c7fe596425d6e4480";
//     let path4_bytes = Vec::from_hex(path4_string).expect("Invalid hex string");
//     let path4 = DynSolValue::Bytes(path4_bytes);

//     let path5_string = "0xf8b18080808080a0348122832f753aead522565162c3bfb3a2ad7fe477bbe2f8ca2696ae35a64f4a8080a0f6c97e9df1e36f10ad1ccfb6deea2a4098a03606332303b755a0d5898151623c80808080a058422ea358cec3f51a60941d504677f94a34a6d5d78623f5d4908085a133fec3a0fac8129af83a04a89a5094dd668fc4a7cfbfb4c01e380892db39ef235fc68b54a0dc9adf2801a8f5bbdc68206741da034ef19078685246daac910603d7db652b0480";
//     let path5_bytes = Vec::from_hex(path5_string).expect("Invalid hex string");
//     let path5 = DynSolValue::Bytes(path5_bytes);

//     let path6_string = "0xf83f9e37fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace9f9e0458c308499d5281a6db800b91d733d70f8ac2ca87a613643b61db36bc1f";
//     let path6_bytes = Vec::from_hex(path6_string).expect("Invalid hex string");
//     let path6 = DynSolValue::Bytes(path6_bytes);

//     let encoded_proof = DynSolValue::Array(vec![path1, path2, path3, path4, path5, path6]);
//     let encoded = encoded_proof.abi_encode();
// }
