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
    pub extrinsic: i32,
    pub intrinsic: i32,
    pub last_extrinsic: i32,
}

impl Score {
    pub fn score(&self) -> i32 {
        self.extrinsic + self.intrinsic
    }

    pub fn accure(&self, delta: i32) -> Score {
        use core::f32::consts::PI;
        use num_traits::Float;

        // f[x] := ArcTan[x/50] * 200 / PI

        let last_extrinsic = self.last_extrinsic + delta;
        let extrinsic = last_extrinsic as f32 / 50.0;
        let extrinsic = extrinsic.atan();
        let extrinsic = extrinsic * 200.0 / PI;

        let extrinsic = (extrinsic.round() * 10.0) as i32 / 10;

        Score {
            extrinsic,
            last_extrinsic,
            ..*self
        }
    }
}
