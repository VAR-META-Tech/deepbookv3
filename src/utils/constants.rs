use std::collections::HashMap;

use crate::types::{Coin, DeepbookPackageIds, Pool};

pub const TESTNET_PACKAGE_IDS: DeepbookPackageIds = DeepbookPackageIds {
    deepbook_package_id: "0xcbf4748a965d469ea3a36cf0ccc5743b96c2d0ae6dee0762ed3eca65fac07f7e",
    registry_id: "0x98dace830ebebd44b7a3331c00750bf758f8a4b17a27380f5bb3fbe68cb984a7",
    deep_treasury_id: "0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb",
};

pub const MAINNET_PACKAGE_IDS: DeepbookPackageIds = DeepbookPackageIds {
    deepbook_package_id: "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809",
    registry_id: "0xaf16199a2dff736e9f07a845f23c5da6df6f756eddb631aed9d24a93efc4549d",
    deep_treasury_id: "0x032abf8948dda67a271bcc18e776dbbcfb0d58c8d288a700ff0d5521e57a1ffe",
};

pub const DEVNET_PACKAGE_IDS: DeepbookPackageIds = DeepbookPackageIds {
    deepbook_package_id: "0xfd76b488f541a06c8747f5932624a411049ee1b89c2370ee042ff39d9c3aa643",
    registry_id: "0xe4ab9ec40cc71134d41360e0981cf6e32ba95e305e24ab105fecb197b8bef831",
    deep_treasury_id: "0x225de6ef76c5dd60b65a7e7724dedfa448276b2be0d95f0b540736d4d9a84755",
};

pub fn get_devnet_coins() -> HashMap<&'static str, Coin> {
    HashMap::from([
        (
            "DEEP",
            Coin {
                address: "0x15cdfe7157290fa17c3c0f75a9663a4cd06c2904876bc077c81f3649de7d481a",
                coin_type: "0x15cdfe7157290fa17c3c0f75a9663a4cd06c2904876bc077c81f3649de7d481a::deep::DEEP",
                scalar: 1_000_000,
            },
        ),
        (
            "SUI",
            Coin {
                address: "0x0000000000000000000000000000000000000000000000000000000000000002",
                coin_type: "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                scalar: 1_000_000_000,
            },
        ),
        (
            "DBUSDC",
            Coin {
                address: "0x983c08cc0866d31081707179f36634b7e8bcaaac3b92bd4a0742daba30f0e6f3",
                coin_type: "0x983c08cc0866d31081707179f36634b7e8bcaaac3b92bd4a0742daba30f0e6f3::dbusdc::DBUSDC",
                scalar: 1_000_000,
            },
        ),
        (
            "DBUSDT",
            Coin {
                address: "0x8636c767476db780a27324b0c961ff0aa783c1953f06bf31fadcd1e3770bde9c",
                coin_type: "0x8636c767476db780a27324b0c961ff0aa783c1953f06bf31fadcd1e3770bde9c::dbusdt::DBUSDT",
                scalar: 1_000_000,
            },
        ),
        (
            "PI",
            Coin {
                address: "0xda3b5dddd274ee7b20de20c5556574f762a6ddc29af8e35a13f91bc60d3e01d3",
                coin_type: "0xda3b5dddd274ee7b20de20c5556574f762a6ddc29af8e35a13f91bc60d3e01d3::pi::PI",
                scalar: 1_000_000,
            },
        ),
    ])
}

pub fn get_devnet_pools() -> HashMap<&'static str, Pool> {
    HashMap::from([
        (
            "DEEP_SUI", // whitelisted pool
            Pool {
                address: "0xfec71b6b2a00bfdb1e41acc43bbcd10c3f4d246973d39cb8bacee0366f79b5d9",
                base_coin: "DEEP",
                quote_coin: "SUI",
            },
        ),
        (
            "SUI_DBUSDC", // whitelisted pool
            Pool {
                address: "0x8b5052f0b83d692c7ee157f8a44d0a3b272f23257130a8b286c49e7fc830ef54",
                base_coin: "SUI",
                quote_coin: "DBUSDC",
            },
        ),
        (
            "DEEP_DBUSDC",
            Pool {
                address: "0x3571bfb4cad9fac64d489a5f1b6b91c66dfc9f21022d65b374a02903c1f2d474",
                base_coin: "DEEP",
                quote_coin: "DBUSDC",
            },
        ),
        (
            "DBUSDT_DBUSDC",
            Pool {
                address: "0x4241fb8f4b8965f8df5a8a80d9c42292d35b7b24d7f3831bb555ffe869337416",
                base_coin: "DBUSDT",
                quote_coin: "DBUSDC",
            },
        ),
        (
            "DBUSDT_SUI", // added price deep point
            Pool {
                address: "0x9e60db786278bbd4912ff6fba077c99cc4f4b984b5c61391e9d9197fb605cbb5",
                base_coin: "DBUSDT",
                quote_coin: "SUI",
            },
        ),
        (
            "PI_SUI",
            Pool {
                address: "0x4b4dbcbc7ae876e77accc279860a65775bacf73932963be1c3b433cd6e859c24",
                base_coin: "PI",
                quote_coin: "SUI",
            },
        ),
    ])
}

pub fn get_testnet_coins() -> HashMap<&'static str, Coin> {
    HashMap::from([
        (
            "DEEP",
            Coin {
                address: "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8",
                coin_type: "0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP",
                scalar: 1_000_000,
            },
        ),
        (
            "SUI",
            Coin {
                address: "0x0000000000000000000000000000000000000000000000000000000000000002",
                coin_type: "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                scalar: 1_000_000_000,
            },
        ),
        (
            "DBUSDC",
            Coin {
                address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7",
                coin_type: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC",
                scalar: 1_000_000,
            },
        ),
        (
            "DBUSDT",
            Coin {
                address: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7",
                coin_type: "0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDT::DBUSDT",
                scalar: 1_000_000,
            },
        ),
        (
            "USDC",
            Coin {
                address: "0xa1ec7fc00a6f40db9693ad1415d0c193ad3906494428cf252621037bd7117e29",
                coin_type: "0xa1ec7fc00a6f40db9693ad1415d0c193ad3906494428cf252621037bd7117e29::usdc::USDC",
                scalar: 1_000_000,
            },
        ),
    ])
}

pub fn get_mainnet_coins() -> HashMap<&'static str, Coin> {
    HashMap::from([
        (
            "DEEP",
            Coin {
                address: "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270",
                coin_type: "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270::deep::DEEP",
                scalar: 1_000_000,
            },
        ),
        (
            "SUI",
            Coin {
                address: "0x0000000000000000000000000000000000000000000000000000000000000002",
                coin_type: "0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI",
                scalar: 1_000_000_000,
            },
        ),
        (
            "USDC",
            Coin {
                address: "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7",
                coin_type: "0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC",
                scalar: 1_000_000,
            },
        ),
        (
            "WUSDC",
            Coin {
                address: "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf",
                coin_type: "0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN",
                scalar: 1_000_000,
            },
        ),
        (
            "WETH",
            Coin {
                address: "0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5",
                coin_type: "0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5::coin::COIN",
                scalar: 100_000_000,
            },
        ),
        (
            "BETH",
            Coin {
                address: "0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29",
                coin_type: "0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29::eth::ETH",
                scalar: 100_000_000,
            },
        ),
        (
            "WBTC",
            Coin {
                address: "0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881",
                coin_type: "0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881::coin::COIN",
                scalar: 100_000_000,
            },
        ),
        (
            "WGIGA",
            Coin {
                address: "0xec32640add6d02a1d5f0425d72705eb76d9de7edfd4f34e0dba68e62ecceb05b",
                coin_type: "0xec32640add6d02a1d5f0425d72705eb76d9de7edfd4f34e0dba68e62ecceb05b::coin::COIN",
                scalar: 100_000,
            },
        ),
    ])
}

pub fn get_testnet_pools() -> HashMap<&'static str, Pool> {
    HashMap::from([
        (
            "DEEP_SUI",
            Pool {
                address: "0x0d1b1746d220bd5ebac5231c7685480a16f1c707a46306095a4c67dc7ce4dcae",
                base_coin: "DEEP",
                quote_coin: "SUI",
            },
        ),
        (
            "SUI_DBUSDC",
            Pool {
                address: "0x520c89c6c78c566eed0ebf24f854a8c22d8fdd06a6f16ad01f108dad7f1baaea",
                base_coin: "SUI",
                quote_coin: "DBUSDC",
            },
        ),
        (
            "DEEP_DBUSDC",
            Pool {
                address: "0xee4bb0db95dc571b960354713388449f0158317e278ee8cda59ccf3dcd4b5288",
                base_coin: "DEEP",
                quote_coin: "DBUSDC",
            },
        ),
        (
            "DBUSDT_DBUSDC",
            Pool {
                address: "0x69cbb39a3821d681648469ff2a32b4872739d2294d30253ab958f85ace9e0491",
                base_coin: "DBUSDT",
                quote_coin: "DBUSDC",
            },
        ),
    ])
}

pub fn get_mainnet_pools() -> HashMap<&'static str, Pool> {
    HashMap::from([
        (
            "DEEP_SUI",
            Pool {
                address: "0xb663828d6217467c8a1838a03793da896cbe745b150ebd57d82f814ca579fc22",
                base_coin: "DEEP",
                quote_coin: "SUI",
            },
        ),
        (
            "SUI_USDC",
            Pool {
                address: "0xe05dafb5133bcffb8d59f4e12465dc0e9faeaa05e3e342a08fe135800e3e4407",
                base_coin: "SUI",
                quote_coin: "USDC",
            },
        ),
        (
            "DEEP_USDC",
            Pool {
                address: "0xf948981b806057580f91622417534f491da5f61aeaf33d0ed8e69fd5691c95ce",
                base_coin: "DEEP",
                quote_coin: "USDC",
            },
        ),
        (
            "WUSDT_USDC",
            Pool {
                address: "0x4e2ca3988246e1d50b9bf209abb9c1cbfec65bd95afdacc620a36c67bdb8452f",
                base_coin: "WUSDT",
                quote_coin: "USDC",
            },
        ),
        (
            "WUSDC_USDC",
            Pool {
                address: "0xa0b9ebefb38c963fd115f52d71fa64501b79d1adcb5270563f92ce0442376545",
                base_coin: "WUSDC",
                quote_coin: "USDC",
            },
        ),
        (
            "BETH_USDC",
            Pool {
                address: "0x1109352b9112717bd2a7c3eb9a416fff1ba6951760f5bdd5424cf5e4e5b3e65c",
                base_coin: "BETH",
                quote_coin: "USDC",
            },
        ),
        (
            "NS_USDC",
            Pool {
                address: "0x0c0fdd4008740d81a8a7d4281322aee71a1b62c449eb5b142656753d89ebc060",
                base_coin: "NS",
                quote_coin: "USDC",
            },
        ),
        (
            "NS_SUI",
            Pool {
                address: "0x27c4fdb3b846aa3ae4a65ef5127a309aa3c1f466671471a806d8912a18b253e8",
                base_coin: "NS",
                quote_coin: "SUI",
            },
        ),
        (
            "TYPUS_SUI",
            Pool {
                address: "0xe8e56f377ab5a261449b92ac42c8ddaacd5671e9fec2179d7933dd1a91200eec",
                base_coin: "TYPUS",
                quote_coin: "SUI",
            },
        ),
        (
            "SUI_AUSD",
            Pool {
                address: "0x183df694ebc852a5f90a959f0f563b82ac9691e42357e9a9fe961d71a1b809c8",
                base_coin: "SUI",
                quote_coin: "AUSD",
            },
        ),
        (
            "AUSD_USDC",
            Pool {
                address: "0x5661fc7f88fbeb8cb881150a810758cf13700bb4e1f31274a244581b37c303c3",
                base_coin: "AUSD",
                quote_coin: "USDC",
            },
        ),
        (
            "DRF_SUI",
            Pool {
                address: "0x126865a0197d6ab44bfd15fd052da6db92fd2eb831ff9663451bbfa1219e2af2",
                base_coin: "DRF",
                quote_coin: "SUI",
            },
        ),
    ])
}
