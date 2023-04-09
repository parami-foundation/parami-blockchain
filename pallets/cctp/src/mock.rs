use crate::*;
use frame_support::traits::ConstU32;
use frame_support::{
    parameter_types, traits::AsEnsureOriginWithArg, traits::GenesisBuild, PalletId,
};

use frame_system::{self as system, EnsureRoot};
use sp_core::{ByteArray, Pair};
use sp_core::{H160, H256};
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup, Keccak256},
};

use frame_support::traits::ConstU128;
use sp_core::crypto::AccountId32;
use sp_io::TestExternalities;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type AssetId = u64;

pub type AccountId = AccountId32;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},

        Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Cctp: crate::{Pallet, Call, Storage, Event<T>},
        Did: parami_did::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type MaxConsumers = ConstU32<16>;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

parameter_types! {
    pub const AssetDeposit: Balance = 100;
    pub const ApprovalDeposit: Balance = 1;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
    type AssetAccountDeposit = ConstU128<10000u128>;
    type Event = Event;
    type Balance = Balance;
    type AssetId = AssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<Self::AccountId>;
    type AssetDeposit = AssetDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

impl parami_did::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type DecentralizedId = H160;
    type Hashing = Keccak256;
    type WeightInfo = ();
    type Transferables = ();
}

parameter_types! {
    pub const CctpPalletId: PalletId = PalletId(*b"prm/cctp");
    pub const CctpChainDomain: u64 = 1;
}

impl crate::Config for Test {
    type Event = Event;
    type CctpAssetId = u64;
    type DomainId = u64;
    type Nonce = u64;
    type Currency = Balances;
    type Assets = Assets;
    type CallOrigin = parami_did::EnsureDid<Self>;
    type PalletId = CctpPalletId;
    type ChainDomain = CctpChainDomain;
}

pub const SIGNING_ACCOUNT: AccountId = AccountId32::new([0x08; 32]);
pub const SIGNING_DID: H160 = H160([0x01; 20]);
pub const ASSET_1: AssetId = 1;
pub const ALICE: AccountId = AccountId32::new([0x01; 32]);
pub const DID_ALICE: H160 = H160([0xff; 20]);
pub const DID_BOB: H160 = H160([0x02; 20]);
pub const BOB_SEED: &str = "/Bob";

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_assets::GenesisConfig::<Test> {
        assets: vec![(1, ALICE, true, 1)],
        accounts: vec![(1, ALICE, 10000)],
        metadata: vec![],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let bob_secret_pair: sp_core::sr25519::Pair =
        sp_core::sr25519::Pair::from_string(BOB_SEED, None).unwrap();
    let bob_account = AccountId32::new(bob_secret_pair.public().as_array_ref().clone());

    parami_did::GenesisConfig::<Test> {
        ids: vec![
            (ALICE, DID_ALICE),
            (SIGNING_ACCOUNT, SIGNING_DID),
            (bob_account, DID_BOB),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 100)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut externalities = TestExternalities::new(t);
    externalities.execute_with(|| System::set_block_number(1));

    externalities
}

pub fn account(id: u8) -> AccountId {
    return AccountId::from([id; 32]);
}
