use crate::{self as parami_clockin, HeightOf};
use frame_support::traits::{ConstU128, ConstU32};
use frame_support::{
    parameter_types, traits::AsEnsureOriginWithArg, traits::GenesisBuild, PalletId,
};
use frame_system::{self as system, EnsureRoot, EnsureSigned};
use sp_core::{sr25519, H160, H256};
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, Keccak256},
};

use parami_primitives::constants::DOLLARS;
use parami_primitives::Network;

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
type Block = system::mocking::MockBlock<Test>;

pub type Extrinsic = TestXt<Call, ()>;

pub const ALICE: sr25519::Public = sr25519::Public([1; 32]);
pub const BOB: sr25519::Public = sr25519::Public([2; 32]);
pub const CHARLIE: sr25519::Public = sr25519::Public([3; 32]);
pub const DAVE: sr25519::Public = sr25519::Public([4; 32]);
pub const EVA: sr25519::Public = sr25519::Public([5; 32]);

pub const DID_ALICE: H160 = H160([0xff; 20]);

pub const NAMESPACE: [u8; 20] = [
    0x06, 0x01, 0x2c, 0x8c, 0xf9, 0x7B, 0xEa, 0xD5, 0xde, 0xAe, 0x23, 0x70, 0x70, 0xF9, 0x58, 0x7f,
    0x8E, 0x7A, 0x26, 0x6d,
];

// Encoded did for "hwtGPq42GojPtyx5ngtSRSpJfjN"
pub const DID_BOB: H160 = H160([
    50, 172, 121, 157, 53, 222, 114, 162, 174, 87, 164, 108, 169, 117, 49, 159, 187, 177, 37, 169,
]);
pub const DID_CHARLIE: H160 = H160([0xdd; 20]);

pub const NEXT_INSTANCE_ID: AssetId = 5000;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Uniques: pallet_uniques::{Pallet, Storage, Event<T>},

        Ocw: parami_ocw::{Pallet},
        Did: parami_did::{Pallet, Call, Storage, Config<T>, Event<T>},
        Nft: parami_nft::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tag: parami_tag::{Pallet, Call, Storage, Config<T>, Event<T>},
        Swap: parami_swap::{Pallet, Call, Storage, Event<T>},
        ClockIn: crate::{Pallet, Call, Storage, Event<T>},
        AssetManager: parami_assetmanager::{Pallet},
    }
);

type AssetId = u32;
type Balance = u128;
type BlockNumber = u64;

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
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = Did;
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

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

impl parami_ocw::Config for Test {}

parameter_types! {
    pub const AssetDeposit: Balance = 100;
    pub const ApprovalDeposit: Balance = 1;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type AssetId = AssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<Self::AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<10000u128>;
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

parameter_types! {
    pub const ClassDeposit: Balance = 0;
    pub const InstanceDeposit: Balance = 0;
    pub const AttributeDepositBase: Balance = 0;
}

impl pallet_uniques::Config for Test {
    type Event = Event;
    type CollectionId = AssetId;
    type ItemId = AssetId;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type CollectionDeposit = ClassDeposit;
    type ItemDeposit = InstanceDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type AttributeDepositBase = AttributeDepositBase;
    type DepositPerByte = MetadataDepositPerByte;
    type StringLimit = StringLimit;
    type KeyLimit = StringLimit;
    type ValueLimit = StringLimit;
    type WeightInfo = ();
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
    type Locker = ();
}

impl parami_did::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type DecentralizedId = H160;
    type Hashing = Keccak256;
    type WeightInfo = ();
    type Transferables = (Nft,);
}

parameter_types! {
    pub const SubmissionFee: Balance = 10;
}

impl parami_tag::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type DecentralizedId = <Self as parami_did::Config>::DecentralizedId;
    type SubmissionFee = SubmissionFee;
    type CallOrigin = parami_did::EnsureDid<Test>;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type WeightInfo = ();
}

parameter_types! {
    pub const DefaultInitialMintingDeposit: Balance = 1_000 * DOLLARS;
    pub const InitialMintingLockupPeriod: BlockNumber = 5;
    pub const InitialMintingValueBase: Balance = 1_000_000 * DOLLARS;
    pub const PendingLifetime: BlockNumber = 5;
    pub const NftPalletId: PalletId = PalletId(*b"prm/nft ");
}

impl parami_assetmanager::Config for Test {
    type AssetId = u32;
}

impl parami_nft::Config for Test {
    type Event = Event;
    type AssetId = AssetId;
    type Assets = Assets;
    type DefaultInitialMintingDeposit = DefaultInitialMintingDeposit;
    type InitialMintingLockupPeriod = InitialMintingLockupPeriod;
    type InitialMintingValueBase = InitialMintingValueBase;
    type Links = ();
    type Nft = Uniques;
    type PalletId = NftPalletId;
    type PendingLifetime = PendingLifetime;
    type StringLimit = StringLimit;
    type Swaps = Swap;
    type WeightInfo = ();
    type UnsignedPriority = ();
    type AssetIdManager = AssetManager;
    type NftId = u32;
}

parameter_types! {
    pub const ClockInPalletId: PalletId = PalletId(*b"prm/nft ");
    pub const ClockInBucketSize: HeightOf<Test> = 10;

}

impl parami_clockin::Config for Test {
    type Event = Event;
    type PalletId = ClockInPalletId;
    type ClockInBucketSize = ClockInBucketSize;
}

parameter_types! {
    pub const SwapPalletId: PalletId = PalletId(*b"prm/swap");
    pub const StakingRewardAmountParam: u128 = 7_000_000 * DOLLARS;
}

impl parami_swap::Config for Test {
    type Event = Event;
    type AssetId = AssetId;
    type Assets = Assets;
    type Currency = Balances;
    type PalletId = SwapPalletId;
    type StakingRewardAmount = StakingRewardAmountParam;
    type Stakes = ();
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (ALICE, 3_000_000 * DOLLARS),
            (BOB, 3_000_000 * DOLLARS),
            (CHARLIE, 3_000_000 * DOLLARS),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    parami_did::GenesisConfig::<Test> {
        ids: vec![(ALICE, DID_ALICE), (BOB, DID_BOB), (CHARLIE, DID_CHARLIE)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    parami_nft::GenesisConfig::<Test> {
        deposit: Default::default(),
        deposits: Default::default(),
        next_instance_id: NEXT_INSTANCE_ID,
        nfts: vec![(0, DID_ALICE, false), (1, DID_ALICE, true)],
        externals: vec![(
            1,
            parami_traits::types::Network::Ethereum,
            NAMESPACE.to_vec(),
            vec![0x01],
            DID_ALICE,
        )],
        validate_endpoints: vec![(Network::Ethereum, "http://etherum.endpoint/example".into())],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let tag1 = b"tag1".to_vec();
    let tag2 = b"tag2".to_vec();

    parami_tag::GenesisConfig::<Test> {
        tag: vec![tag1.clone(), tag2.clone()],
        personas: vec![
            (DID_BOB, tag1, parami_tag::types::Score::new(5)),
            (DID_BOB, tag2, parami_tag::types::Score::new(7)),
        ],
        ..Default::default()
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
