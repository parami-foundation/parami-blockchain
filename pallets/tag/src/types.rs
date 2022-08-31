use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Metadata<V, D, N> {
    pub creator: D,
    pub created: N,
    pub tag: V,
}

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SingleMetricScore {
    pub current_score: i32,
    pub last_input: i32,
}

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Score {
    extrinsic: u8,
    intrinsic: u8,
}

impl Score {
    pub fn new(intrinsic: u8) -> Score {
        assert!(intrinsic <= 50);
        return Score {
            intrinsic,
            extrinsic: 0,
        };
    }

    pub fn score(&self) -> i32 {
        (self.extrinsic + self.intrinsic) as i32
    }

    pub fn accure_extrinsic(&self, rating: u8) -> Score {
        assert!(rating <= 5);

        let prev_extrinsic = self.extrinsic;
        let scaled_rating = 10 * rating;
        let extrinsic = (0.8 * prev_extrinsic as f32 + 0.2 * scaled_rating as f32) as u8;

        return Score { extrinsic, ..*self };
    }

    pub fn with_intrinsic(&self, intrinsic: u8) -> Score {
        return Score { intrinsic, ..*self };
    }
}
