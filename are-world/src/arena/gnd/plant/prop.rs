use crate::arena::gnd::Environment;
use crate::math;
use crate::meta::types::*;

pub const DETAIL: [PlantDetail; 2] = [
    PlantDetail {
        name: "tree",
        max_energy: 500000,
        grow: 2,
        gen: GenerateDetail::Gauss {
            humid_deviation: 15.0,
            humid_center: 128.0,
            max_possibility: 0.1,
        },
    },
    PlantDetail {
        max_energy: 50000,
        name: "grass",
        grow: 1,
        gen: GenerateDetail::Gauss {
            humid_deviation: 50.0,
            humid_center: 50.0,
            max_possibility: 0.8,
        },
    },
];

pub struct PlantDetail {
    grow: EnergyT,
    pub max_energy: EnergyT,
    pub name: &'static str,

    pub gen: GenerateDetail,
}

impl PlantDetail {
    #[inline]
    pub fn growth(&self, env: &Environment) -> EnergyT {
        todo!()
    }
}

pub enum GenerateDetail {
    Gauss {
        humid_deviation: f32,
        humid_center: f32,
        max_possibility: f32,
    },
}

impl GenerateDetail {
    #[inline]
    pub fn possibility(&self, env: &Environment) -> f32 {
        match self {
            GenerateDetail::Gauss {
                humid_deviation,
                humid_center,
                max_possibility,
            } => {
                let d = math::functions::gauss::density(
                    env.humid.into(),
                    *humid_center,
                    *humid_deviation,
                ) /*height*/ * 1.0 /*width*/;
                d / math::functions::gauss::density(0.0, 0.0, *humid_deviation) * max_possibility
            }
        }
    }
}
