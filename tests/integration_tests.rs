//! Business Rates Calculation Engine - Integration Test Runner
//!
//! Organized into 10 statutory test suites (Suites A through J) directly matching
//! the statutory specifications in business-rates-engine-spec-and-tests.md.

#[path = "integration_tests/common.rs"]
mod common;

#[path = "integration_tests/suite_a_scenarios.rs"]
mod suite_a_scenarios;

#[path = "integration_tests/suite_b_multi_occupancy.rs"]
mod suite_b_multi_occupancy;

#[path = "integration_tests/suite_c_i18n.rs"]
mod suite_c_i18n;

#[path = "integration_tests/suite_d_api_and_parameters.rs"]
mod suite_d_api_and_parameters;

#[path = "integration_tests/suite_e_england_schemes.rs"]
mod suite_e_england_schemes;

#[path = "integration_tests/suite_f_wales_schemes.rs"]
mod suite_f_wales_schemes;

#[path = "integration_tests/suite_g_scotland_schemes.rs"]
mod suite_g_scotland_schemes;

#[path = "integration_tests/suite_h_cross_jurisdictional.rs"]
mod suite_h_cross_jurisdictional;

#[path = "integration_tests/suite_i_ui_and_security.rs"]
mod suite_i_ui_and_security;

#[path = "integration_tests/suite_j_historical.rs"]
mod suite_j_historical;
