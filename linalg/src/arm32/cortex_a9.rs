use crate::frame::mmm::cost_model::CostModel;
pub fn models() -> Vec<(&'static str, CostModel)> {
vec!(
("generic_f32_4x4", CostModel { mr: 4, nr: 4,
intercept: 0.0000005456745230237817,
coef: vec!(4.2633479434259085e-7, 2.4244627282269802e-8, -4.129783106386242e-8, -2.4065291957142746e-10, 1.0552331177638251e-8, -2.2289729987399014e-10, 8.676297552715139e-8, 1.2952472209907063e-7, 2.4056445374128295e-8, 1.2464368141270966e-8, -1.591964935450874e-7),
}),
("armv7neon_mmm_f32_8x4_cortexa7", CostModel { mr: 8, nr: 4,
intercept: 0.0000004880123921712441,
coef: vec!(4.3015538039933967e-7, 3.727799505458647e-8, -3.445876231142306e-8, -1.4600514777645331e-10, 1.9393366215227735e-8, -8.021399314343384e-11, 1.009025985057537e-7, 1.6238119397399134e-7, 1.5837906176237072e-8, 2.709462114758536e-8, -2.373203415208295e-7),
}),
("armv7neon_mmm_f32_8x6_cortexa7", CostModel { mr: 8, nr: 6,
intercept: 0.00000040748064879353097,
coef: vec!(4.746986024720382e-7, 4.848804923656522e-8, 8.119711625166692e-8, -7.700960637198129e-10, 1.390663295554658e-7, -5.517277662901602e-10, 1.2077214755665708e-7, 1.7183941604663407e-7, 3.807677657066438e-8, 1.7799712082386767e-8, -3.631995530662536e-7),
}),
("armv7neon_mmm_f32_8x4_cortexa9", CostModel { mr: 8, nr: 4,
intercept: 0.0000003421358675744475,
coef: vec!(4.3515015978251394e-7, 2.625638322252613e-8, 1.2223776203711873e-7, -2.304408953332742e-10, 1.1860924624916627e-7, -1.318716271905676e-10, 9.424310009678724e-8, 1.3836218326698572e-7, 1.6661002908199407e-8, 3.102987930561987e-8, -2.150917081435957e-7),
}),
("armv7neon_mmm_f32_8x6_cortexa9", CostModel { mr: 8, nr: 6,
intercept: 0.0000003495708425302802,
coef: vec!(4.812608093012768e-7, 3.565331474138168e-8, 1.6241460365076086e-7, -4.324861502649183e-10, 2.6595237513661234e-7, -2.2251003358380796e-9, 1.208013944173288e-7, 1.6388123904134127e-7, 3.7194235127080764e-8, 1.6290616122683435e-8, -3.2877745305523384e-7),
}),
("armv7neon_mmm_f32_8x4_generic", CostModel { mr: 8, nr: 4,
intercept: 0.00000016563946973412396,
coef: vec!(4.37862949708138e-7, 2.629850002138698e-8, 2.899757648016548e-7, -5.975436293384518e-10, 3.707297438313926e-7, -4.832133585638963e-10, 1.0307336900426979e-7, 1.6399207285584852e-7, 1.6549874839963914e-8, 2.2354730036359145e-8, -2.227087560357608e-7),
}),
("armv7neon_mmm_f32_8x6_generic", CostModel { mr: 8, nr: 6,
intercept: -0.00000006869100438269672,
coef: vec!(4.847756709705402e-7, 3.5884272087009983e-8, 6.60929253627602e-7, -1.933229148754563e-9, 7.476935844390713e-7, -2.9972141315669673e-9, 1.1432050790753746e-7, 2.052268123786658e-7, 3.7925062179901675e-8, 1.1441484587248824e-8, -3.838542147520336e-7),
}),
)}