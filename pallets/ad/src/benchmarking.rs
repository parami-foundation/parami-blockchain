use crate::BalanceOf;
#[allow(unused)]
use crate::Pallet as Ad;
use crate::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use parami_advertiser::Pallet as Advertiser;
use parami_did::Pallet as Did;
use parami_nft::Pallet as Nft;
use parami_primitives::constants::DOLLARS;
use parami_swap::Pallet as Swap;
use parami_tag::Pallet as Tag;
use sp_runtime::traits::{Bounded, Saturating, Zero};

fn prepare_nft<T>(caller: &T::AccountId) -> <T as parami_nft::Config>::AssetId
where
    T: parami_advertiser::Config,
    T: parami_did::Config,
    T: parami_nft::Config,
    T: parami_tag::Config,
    T: parami_swap::Config,
    T: crate::Config,
{
    let balance: BalanceOf<T> = (1000 * DOLLARS)
        .try_into()
        .map_err(|_| "balance conversion")
        .unwrap();

    let nft_id_u32 = 1u32;
    let nft_id = nft_id_u32.into();

    let kol: T::AccountId = account("kol", 1, 1);
    let _ = Did::<T>::register(RawOrigin::Signed(kol.clone()).into(), None);

    let _kick_res = Nft::<T>::kick(RawOrigin::Signed(kol.clone()).into());
    let _back_res = Nft::<T>::back(RawOrigin::Signed(caller.clone()).into(), nft_id, balance);
    let _mint_res = Nft::<T>::mint(
        RawOrigin::Signed(kol).into(),
        nft_id,
        b"Test Token".to_vec(),
        b"XTT".to_vec(),
        Some(1_000_000_000u32.into()),
    );

    let _swap_res = Swap::<T>::buy_tokens(
        RawOrigin::Signed(caller.clone()).into(),
        nft_id_u32.into(),
        (500_000 * DOLLARS)
            .try_into()
            .map_err(|_| "balance conversion")
            .unwrap(),
        (3000 * DOLLARS)
            .try_into()
            .map_err(|_| "balance conversion")
            .unwrap(),
        100000000u32
            .try_into()
            .map_err(|_| "block number conversion")
            .unwrap(),
    );
    // Nft::<T>::claim(RawOrigin::Signed(caller.clone()).into(), nft_id);

    nft_id
}

fn prepare_ad<T>() -> (T::AccountId, <T as frame_system::Config>::Hash)
where
    T: parami_advertiser::Config,
    T: parami_did::Config,
    T: parami_nft::Config,
    T: parami_tag::Config,
    T: crate::Config,
{
    let caller: T::AccountId = whitelisted_caller();
    let payout_base: BalanceOf<T> = 1u32.into();
    let payout_min: BalanceOf<T> = 1u32.into();
    let payout_max: BalanceOf<T> = 10u32.into();

    <T as parami_did::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

    let balance: BalanceOf<T> = (1000 * DOLLARS)
        .try_into()
        .map_err(|_| "balance conversion")
        .unwrap();

    let _ = Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), None);
    Advertiser::<T>::deposit(RawOrigin::Signed(caller.clone()).into(), balance).unwrap();
    let _ = Tag::<T>::force_create(RawOrigin::Root.into(), vec![1u8; 6]);

    Ad::<T>::create(
        RawOrigin::Signed(caller.clone()).into(),
        vec![vec![1u8; 6]],
        Default::default(),
        1,
        HeightOf::<T>::max_value(),
        payout_base,
        payout_min,
        payout_max,
        Default::default(),
    )
    .unwrap();

    let ad = <Metadata<T>>::iter_keys().next().unwrap();

    (caller, ad)
}

benchmarks! {
    where_clause {
        where
        T: parami_advertiser::Config,
        T: parami_did::Config,
        T: parami_nft::Config,
        T: parami_tag::Config,
        T: parami_swap::Config,
        T: crate::Config
    }

    create {
        // TODO: add back variables

        let caller: T::AccountId = whitelisted_caller();

        let payout_base: BalanceOf<T> = 1u32.into();
        let payout_min: BalanceOf<T> = 0u32.into();
        let payout_max: BalanceOf<T> = 10u32.into();

        <T as parami_did::Config>::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        let balance: BalanceOf<T> = (1000 * DOLLARS)
            .try_into()
            .map_err(|e| "balance conversion")
            .unwrap();

        let _ = Did::<T>::register(RawOrigin::Signed(caller.clone()).into(), None);
        Advertiser::<T>::deposit(RawOrigin::Signed(caller.clone()).into(), balance)?;
        let _ = Tag::<T>::force_create(RawOrigin::Root.into(), vec![1u8; 6]);

    }: _(RawOrigin::Signed(caller), vec![vec![1u8; 6]], vec![0u8; 500], 1, HeightOf::<T>::max_value(), payout_base, payout_min, payout_max, Default::default())
    verify {
        assert_ne!(<Metadata<T>>::iter_values().next(), None);
    }

    update_reward_rate {
        let (caller, ad) = prepare_ad::<T>();
    }: _(RawOrigin::Signed(caller), ad, 100)
    verify {
        let ad = <Metadata<T>>::get(&ad).unwrap();
        assert_eq!(ad.reward_rate, 100);
    }

    update_tags {
        let (caller, ad) = prepare_ad::<T>();

    }: _(RawOrigin::Signed(caller), ad, vec![vec![1u8; 6]])
    verify {
        assert_eq!(Tag::<T>::tags_of(&ad).len(), 1);
    }

    bid_with_fraction {
        let (caller, ad) = prepare_ad::<T>();
        let nft_id = prepare_nft::<T>(&caller);

    }: _(RawOrigin::Signed(caller.clone()), ad, nft_id, 1000u32.into(), None, None)
    verify {
        assert_ne!(<SlotOf<T>>::get(nft_id), None);
    }

    add_budget {
        let (caller, ad) = prepare_ad::<T>();
        let nft_id = prepare_nft::<T>(&caller);

        let initial_bid: BalanceOf<T> = 1000u32.into();
        let budget: BalanceOf<T> = 100u32.into();

        Ad::<T>::bid_with_fraction(RawOrigin::Signed(caller.clone()).into(), ad, nft_id, initial_bid, None, None)?;
    }: _(RawOrigin::Signed(caller.clone()), ad, nft_id, budget, None, None)
    verify {
        let nft = <SlotOf<T>>::get(nft_id).unwrap();
        assert_eq!(<T as parami_nft::Config>::Assets::balance(nft.fraction_id, &nft.budget_pot), initial_bid.saturating_add(budget));
    }

    pay {
        let (caller, ad) = prepare_ad::<T>();
        let nft_id = prepare_nft::<T>(&caller);

        let _ = Ad::<T>::bid_with_fraction(RawOrigin::Signed(caller.clone()).into(), ad, nft_id, 1000u32.into(), None, None);

        let visitor: T::AccountId = account("visitor", 2, 2);
        let _ = Did::<T>::register(RawOrigin::Signed(visitor.clone()).into(), None);
        let did = Did::<T>::did_of(&visitor).unwrap();

    }: _(RawOrigin::Signed(caller.clone()), ad, nft_id, did, vec![(vec![1u8; 6], 5)], None)
    verify {
        assert_ne!(<T as parami_nft::Config>::Assets::balance(nft_id, &visitor), Zero::zero());
    }

    impl_benchmark_test_suite!(Ad, crate::mock::new_test_ext(), crate::mock::Test);
}
