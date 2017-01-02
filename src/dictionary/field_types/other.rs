// Copyright 2016 James Bendig. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under:
//   the MIT license
//     <LICENSE-MIT or https://opensource.org/licenses/MIT>
//   or the Apache License, Version 2.0
//     <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>,
// at your option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Write;
use std::str::FromStr;

use field_type::FieldType;
use message::SetValueError;

//Enumerated Fields (Sorted Alphabetically)

#[derive(Clone,Debug,PartialEq)]
pub enum ApplVerID {
    FIX27,
    FIX30,
    FIX40,
    FIX41,
    FIX42,
    FIX43,
    FIX44,
    FIX50,
    FIX50SP1,
    FIX50SP2,
}

define_enum_field_type!(NOT_REQUIRED, ApplVerID, ApplVerIDFieldType {
    ApplVerID::FIX27 => b"0",
    ApplVerID::FIX30 => b"1",
    ApplVerID::FIX40 => b"2",
    ApplVerID::FIX41 => b"3",
    ApplVerID::FIX42 => b"4",
    ApplVerID::FIX43 => b"5",
    ApplVerID::FIX44 => b"6",
    ApplVerID::FIX50 => b"7",
    ApplVerID::FIX50SP1 => b"8",
    ApplVerID::FIX50SP2 => b"9",
} MUST_BE_STRING);

#[derive(Clone,Debug,PartialEq)]
pub enum BusinessRejectReason {
    Other,
    UnknownID,
    UnknownSecurity,
    UnsupportedMessageType,
    ApplicationNotAvailable,
    ConditionallyRequiredFieldMissing,
    NotAuthorized,
    DeliverToFirmNotAvailableAtThisTime,
    InvalidPriceIncrement,
}

define_enum_field_type!(REQUIRED, BusinessRejectReason, BusinessRejectReasonFieldType {
    BusinessRejectReason::Other => b"0",
    BusinessRejectReason::UnknownID => b"1",
    BusinessRejectReason::UnknownSecurity => b"2",
    BusinessRejectReason::UnsupportedMessageType => b"3",
    BusinessRejectReason::ApplicationNotAvailable => b"4",
    BusinessRejectReason::ConditionallyRequiredFieldMissing => b"5",
    BusinessRejectReason::NotAuthorized => b"6",
    BusinessRejectReason::DeliverToFirmNotAvailableAtThisTime => b"7",
    BusinessRejectReason::InvalidPriceIncrement => b"18",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum ComplexEventCondition {
    And,
    Or
}

define_enum_field_type!(NOT_REQUIRED, ComplexEventCondition, ComplexEventConditionFieldType {
    ComplexEventCondition::And => b"1",
    ComplexEventCondition::Or => b"2",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum ComplexEventPriceBoundaryMethod {
    LessThanComplexEventPrice,
    LessThanOrEqualToComplexEventPrice,
    EqualToComplexEventPrice,
    GreaterThanOrEqualToComplexEventPrice,
    GreaterThanComplexEventPrice,
}

define_enum_field_type!(NOT_REQUIRED, ComplexEventPriceBoundaryMethod, ComplexEventPriceBoundaryMethodFieldType {
    ComplexEventPriceBoundaryMethod::LessThanComplexEventPrice => b"1",
    ComplexEventPriceBoundaryMethod::LessThanOrEqualToComplexEventPrice => b"2",
    ComplexEventPriceBoundaryMethod::EqualToComplexEventPrice => b"3",
    ComplexEventPriceBoundaryMethod::GreaterThanOrEqualToComplexEventPrice => b"4",
    ComplexEventPriceBoundaryMethod::GreaterThanComplexEventPrice => b"5",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum ComplexEventPriceTimeType {
    Expiration,
    Immediate,
    SpecifiedDateTime,
}

define_enum_field_type!(NOT_REQUIRED, ComplexEventPriceTimeType, ComplexEventPriceTimeTypeFieldType {
    ComplexEventPriceTimeType::Expiration => b"1",
    ComplexEventPriceTimeType::Immediate => b"2",
    ComplexEventPriceTimeType::SpecifiedDateTime => b"3",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum ComplexEventType {
    Capped,
    Trigger,
    KnockInUp,
    KnockInDown,
    KnockOutUp,
    KnockOutDown,
    Underlying,
    ResetBarrier,
    RollingBarrier,
}

define_enum_field_type!(REQUIRED, ComplexEventType, ComplexEventTypeFieldType {
    ComplexEventType::Capped => b"1",
    ComplexEventType::Trigger => b"2",
    ComplexEventType::KnockInUp => b"3",
    ComplexEventType::KnockInDown => b"4",
    ComplexEventType::KnockOutUp => b"5",
    ComplexEventType::KnockOutDown => b"6",
    ComplexEventType::Underlying => b"7",
    ComplexEventType::ResetBarrier => b"8",
    ComplexEventType::RollingBarrier => b"9",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum ContractMultiplierUnit {
    Shares,
    Hours,
    Days,
}

define_enum_field_type!(NOT_REQUIRED, ContractMultiplierUnit, ContractMultiplierUnitFieldType {
    ContractMultiplierUnit::Shares => b"0",
    ContractMultiplierUnit::Hours => b"1",
    ContractMultiplierUnit::Days => b"2",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum CPProgram {
    _3A3,
    _42,
    Other,
    Reserved100Plus(i64),
}

define_enum_field_type_with_reserved!(NOT_REQUIRED, CPProgram, CPProgramFieldType {
    CPProgram::_3A3 => 1,
    CPProgram::_42 => 2,
    CPProgram::Other => 99,
} CPProgram::Reserved100Plus => WITH_MINIMUM 100);

#[derive(Clone,PartialEq)]
pub enum EmailType {
    New,
    Reply,
    AdminReply,
}

define_enum_field_type!(REQUIRED, EmailType, EmailTypeFieldType {
    EmailType::New => b"0",
    EmailType::Reply => b"1",
    EmailType::AdminReply => b"2",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum EventType {
    Put,
    Call,
    Tender,
    SinkingFundCall,
    Activation,
    Inactivation,
    LastEligibleTradeDate,
    SwapStartDate,
    SwapEndDate,
    SwapRollDate,
    SwapNextStartDate,
    SwapNextRollDate,
    FirstDeliveryDate,
    LastDeliveryDate,
    InitialInventoryDueDate,
    FinalInventoryDueDate,
    FirstIntentDate,
    LastIntentDate,
    PositionRemovalDate,
    Other,
    Reserved100Plus(i64),
}

define_enum_field_type_with_reserved!(REQUIRED, EventType, EventTypeFieldType {
    EventType::Put => 1,
    EventType::Call => 2,
    EventType::Tender => 3,
    EventType::SinkingFundCall => 4,
    EventType::Activation => 5,
    EventType::Inactivation => 6,
    EventType::LastEligibleTradeDate => 7,
    EventType::SwapStartDate => 8,
    EventType::SwapEndDate => 9,
    EventType::SwapRollDate => 10,
    EventType::SwapNextStartDate => 11,
    EventType::SwapNextRollDate => 12,
    EventType::FirstDeliveryDate => 13,
    EventType::LastDeliveryDate => 14,
    EventType::InitialInventoryDueDate => 15,
    EventType::FinalInventoryDueDate => 16,
    EventType::FirstIntentDate => 17,
    EventType::LastIntentDate => 18,
    EventType::PositionRemovalDate => 19,
    EventType::Other => 99,
} EventType::Reserved100Plus => WITH_MINIMUM 100);

#[derive(Clone,PartialEq)]
pub enum ExerciseStyle {
    European,
    American,
    Bermuda,
}

define_enum_field_type!(NOT_REQUIRED, ExerciseStyle, ExerciseStyleFieldType {
    ExerciseStyle::European => b"0",
    ExerciseStyle::American => b"1",
    ExerciseStyle::Bermuda => b"2",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum FlowScheduleType {
    NERCEasternOffPeak,
    NERCWesternOffPeak,
    NERCCalendar,
    NERCEasternPeak,
    NERCWesternPeak,
    Reserved100Plus(i64),
}

define_enum_field_type_with_reserved!(NOT_REQUIRED, FlowScheduleType, FlowScheduleTypeFieldType {
    FlowScheduleType::NERCEasternOffPeak => 0,
    FlowScheduleType::NERCWesternOffPeak => 1,
    FlowScheduleType::NERCCalendar => 2,
    FlowScheduleType::NERCEasternPeak => 3,
    FlowScheduleType::NERCWesternPeak => 4,
} FlowScheduleType::Reserved100Plus => WITH_MINIMUM 100);

#[derive(Clone,PartialEq)]
pub enum HandlInst {
    AutomatedExecutionOrderPrivateNoBrokerIntervention,
    AutomatedExecutionOrderPublicBrokerInterventionOK,
    ManualOrderBestExecution,
}

define_enum_field_type!(NOT_REQUIRED, HandlInst, HandlInstFieldType {
    HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention => b"1",
    HandlInst::AutomatedExecutionOrderPublicBrokerInterventionOK => b"2",
    HandlInst::ManualOrderBestExecution => b"3",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum InstrmtAssignmentMethod {
    Random,
    ProRata,
}

define_enum_field_type!(NOT_REQUIRED, InstrmtAssignmentMethod, InstrmtAssignmentMethodFieldType {
    InstrmtAssignmentMethod::Random => b"R",
    InstrmtAssignmentMethod::ProRata => b"P",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum Issuer {
    CouncilOfEurope,
    DeutscheAusgleichsbank,
    EuropeanBankForReconstructionAndDevelopment,
    EuropeanInvestmentBank,
    Hessen,
    KreditanstaltFuerWiederaufbau,
    LandwirtschaftlicheRentenbank,
    NordRheinWestfalenNRW,
    SachsenAnhalt,
    AustrianTreasuryBill,
    AustrianGovernmentBond,
    AustrianBundesobligation, //OBL
    AustrianBundesschatzscheine,
    AustrianGovernmentInternationalBond,
    RAGBCouponStrip, //Austrian
    RAGBPrincipalStrip, //Austrian
    AustriaMediumTermBill,
    BelgianTreasuryBill,
    BelgianGovernmentBond,
    BelgianGovernmentInternationalBond,
    BelgianStrip,
    BelgianPrincipalStrip,
    DanishTreasuryBill,
    DanishGovernmentBond,
    DanishGovernmentInternationalBond,
    FinnishTreasuryBill,
    FinnishGovernmentBond,
    FinnishGovernmentInternationalBond,
    FinnishHousingBond,
    FrenchFixedRateShortTermDiscountTreasuryBills, //BTF
    FrenchFixedRateTreasuryNotes, //BTAN
    FrenchTreasuryBonds, //OAT
    FrenchTreasuryBondsPrincipalSTRIPS, //OAT
    FrenchTreasuryBondsCouponSTRIPS, //OAT
    SocialSecurityDebtRepaymentFund, //French
    GermanTreasuryBill,
    GermanFederalTreasuryBill, //DM Ccy
    GermanTwoYearNotes,
    GermanFinancingTreasuryNotes, //DM Ccy
    GermanGovernmentBond,
    GermanGovernmentBondPrincipalSTRIPS,
    GermanGovernmentBondCouponSTRIPS,
    GermanFiveYearBonds,
    GermanUnityFundDBR, //S (only 2)
    GermanUnityFund, //BKO (None)
    GermanFederalPost, //BUNDESPOST
    GermanFederalRailroad, //BUNDESBAHN
    TreuhandAgencyBonds,
    TreuhandAgencyObligations, //All matured
    GermanRetributionFund, //Only 2 sinking funds
    EuropeanRecoveryProgramSpecialFunds, //German only 2)
    Bundeskassenscheine, //1 matured
    HellenicRepublicTreasuryBill,
    HellenicRepublicGovernmentBond,
    HellenicRepublicGovernmentInternationalBond,
    HellenicRepublicGovernmentBondCouponSTRIPS,
    HellenicRepublicGovernmentBondResidualSTRIPS,
    IrishGovernmentBond,
    IrishGovernmentInternationalBond,
    ItalianTreasuryBill,
    ItalianGovernmentBond,
    ItalianTreasuryCertificate,
    ItalianZeroCouponBonds,
    ItalianGovernmentBondsIssuedInEUR, //Matured
    ItalianGovernmentBondsWithPutOption, //All matured
    ItalianInternationalBonds,
    ItalianGovernmentBondCouponSTRIPS,
    ItalianGovernmentBondResidualSTRIPS,
    LuxembourgeoisGovernmentBond,
    DutchGovernmentBond,
    DutchPrincipalStrip,
    DutchStrip,
    DutchTreasuryCertificate,
    DutchBankCertificate, //All matured
    NorwegianTreasuryBill,
    NorwegianGovernmentBond,
    NorwegianGovernmentInternationalBond, //NOK
    PortugueseTreasuryBills,
    PortugueseGovernmentBond,
    PortugueseGovernmentInternationalBond,
    SpanishGovernmentBond,
    SpanishGovernmentBondCouponStrips,
    SpanishGovernmentBondPrincipalStrips,
    SpanishGovernmentInternationalBond,
    SpanishLetrasDelTesoro,
    SwedishTreasuryBill,
    SwedishGovernmentBond,
    SwedishGovernmentInternationalBond, //SEK
    SwedishGovernmentBondCouponStrip,
    SwedishGovernmentBondResidualStrip,
    SwissTreasuryBill,
    SwissGovernmentBond,
    GenevaTreasuryBill, //CHF
    UnitedKingdomGBPOrEURTreasuryBill,
    UnitedKingdomGiltBond,
    UnitedKingdomGiltBondCouponSTRIPS,
    UnitedKingdomGiltBondResidualSTRIPS,
    UnitedKingdomInternationalBond,
    BankOfEnglandEURBill,
    BankOfEnglandEURNote,
}

define_enum_field_type!(NOT_REQUIRED, Issuer, IssuerFieldType {
    Issuer::CouncilOfEurope => b"COE",
    Issuer::DeutscheAusgleichsbank => b"DTA",
    Issuer::EuropeanBankForReconstructionAndDevelopment => b"EBRD",
    Issuer::EuropeanInvestmentBank => b"EIB",
    Issuer::Hessen => b"HESLAN",
    Issuer::KreditanstaltFuerWiederaufbau => b"KFW",
    Issuer::LandwirtschaftlicheRentenbank => b"LANREN",
    Issuer::NordRheinWestfalenNRW => b"NORWES",
    Issuer::SachsenAnhalt => b"SACHAN",
    Issuer::AustrianTreasuryBill => b"RATB",
    Issuer::AustrianGovernmentBond => b"RAGB",
    Issuer::AustrianBundesobligation => b"AOBL",
    Issuer::AustrianBundesschatzscheine => b"RABSS",
    Issuer::AustrianGovernmentInternationalBond => b"AUST",
    Issuer::RAGBCouponStrip => b"RAGBS",
    Issuer::RAGBPrincipalStrip => b"RAGBR",
    Issuer::AustriaMediumTermBill => b"RAMTB",
    Issuer::BelgianTreasuryBill => b"BGTB",
    Issuer::BelgianGovernmentBond => b"BGB",
    Issuer::BelgianGovernmentInternationalBond => b"BELG",
    Issuer::BelgianStrip => b"OLOS",
    Issuer::BelgianPrincipalStrip => b"OLOR",
    Issuer::DanishTreasuryBill => b"DGTB",
    Issuer::DanishGovernmentBond => b"DGB",
    Issuer::DanishGovernmentInternationalBond => b"DENK",
    Issuer::FinnishTreasuryBill => b"RFTB",
    Issuer::FinnishGovernmentBond => b"RFGB",
    Issuer::FinnishGovernmentInternationalBond => b"FINL",
    Issuer::FinnishHousingBond => b"FNHF",
    Issuer::FrenchFixedRateShortTermDiscountTreasuryBills => b"BTF",
    Issuer::FrenchFixedRateTreasuryNotes => b"BTNS",
    Issuer::FrenchTreasuryBonds => b"FRTR",
    Issuer::FrenchTreasuryBondsPrincipalSTRIPS => b"FRTRR",
    Issuer::FrenchTreasuryBondsCouponSTRIPS => b"FRTRS",
    Issuer::SocialSecurityDebtRepaymentFund => b"CADES",
    Issuer::GermanTreasuryBill => b"BUBILL",
    Issuer::GermanFederalTreasuryBill => b"DBSB",
    Issuer::GermanTwoYearNotes => b"BKO",
    Issuer::GermanFinancingTreasuryNotes => b"FSDB",
    Issuer::GermanGovernmentBond => b"DBR",
    Issuer::GermanGovernmentBondPrincipalSTRIPS => b"DBRR",
    Issuer::GermanGovernmentBondCouponSTRIPS => b"DBRS",
    Issuer::GermanFiveYearBonds => b"OBL",
    Issuer::GermanUnityFundDBR => b"DBRUF",
    Issuer::GermanUnityFund => b"BKOUF",
    Issuer::GermanFederalPost => b"DBP",
    Issuer::GermanFederalRailroad => b"DBB",
    Issuer::TreuhandAgencyBonds => b"THA",
    Issuer::TreuhandAgencyObligations => b"TOBL",
    Issuer::GermanRetributionFund => b"ENTFND",
    Issuer::EuropeanRecoveryProgramSpecialFunds => b"GERP",
    Issuer::Bundeskassenscheine => b"BUNKASS",
    Issuer::HellenicRepublicTreasuryBill => b"GTB",
    Issuer::HellenicRepublicGovernmentBond => b"GGB",
    Issuer::HellenicRepublicGovernmentInternationalBond => b"GREECE",
    Issuer::HellenicRepublicGovernmentBondCouponSTRIPS => b"GGBSTP",
    Issuer::HellenicRepublicGovernmentBondResidualSTRIPS => b"GGBRES",
    Issuer::IrishGovernmentBond => b"IRISH",
    Issuer::IrishGovernmentInternationalBond => b"IRELND",
    Issuer::ItalianTreasuryBill => b"BOTS",
    Issuer::ItalianGovernmentBond => b"BTPS",
    Issuer::ItalianTreasuryCertificate => b"CCTS",
    Issuer::ItalianZeroCouponBonds => b"ICTZ",
    Issuer::ItalianGovernmentBondsIssuedInEUR => b"CTES",
    Issuer::ItalianGovernmentBondsWithPutOption => b"CTOS",
    Issuer::ItalianInternationalBonds => b"ITALY",
    Issuer::ItalianGovernmentBondCouponSTRIPS => b"BTPSS",
    Issuer::ItalianGovernmentBondResidualSTRIPS => b"BTPSR",
    Issuer::LuxembourgeoisGovernmentBond => b"LGB",
    Issuer::DutchGovernmentBond => b"NETHER",
    Issuer::DutchPrincipalStrip => b"NETHRR",
    Issuer::DutchStrip => b"NETHRS",
    Issuer::DutchTreasuryCertificate => b"DTB",
    Issuer::DutchBankCertificate => b"NBC",
    Issuer::NorwegianTreasuryBill => b"NGTB",
    Issuer::NorwegianGovernmentBond => b"NGB",
    Issuer::NorwegianGovernmentInternationalBond => b"NORWAY",
    Issuer::PortugueseTreasuryBills => b"PORTB",
    Issuer::PortugueseGovernmentBond => b"PGB",
    Issuer::PortugueseGovernmentInternationalBond => b"PORTUG",
    Issuer::SpanishGovernmentBond => b"SPGB",
    Issuer::SpanishGovernmentBondCouponStrips => b"SPGBS",
    Issuer::SpanishGovernmentBondPrincipalStrips => b"SPGBR",
    Issuer::SpanishGovernmentInternationalBond => b"SPAIN",
    Issuer::SpanishLetrasDelTesoro => b"SGLT",
    Issuer::SwedishTreasuryBill => b"SWTB",
    Issuer::SwedishGovernmentBond => b"SGB",
    Issuer::SwedishGovernmentInternationalBond => b"SWED",
    Issuer::SwedishGovernmentBondCouponStrip => b"SGBS",
    Issuer::SwedishGovernmentBondResidualStrip => b"SGBR",
    Issuer::SwissTreasuryBill => b"SWISTB",
    Issuer::SwissGovernmentBond => b"SWISS",
    Issuer::GenevaTreasuryBill => b"GENTB",
    Issuer::UnitedKingdomGBPOrEURTreasuryBill => b"UKTB",
    Issuer::UnitedKingdomGiltBond => b"UKT",
    Issuer::UnitedKingdomGiltBondCouponSTRIPS => b"UKTS",
    Issuer::UnitedKingdomGiltBondResidualSTRIPS => b"UKTR",
    Issuer::UnitedKingdomInternationalBond => b"UKIN",
    Issuer::BankOfEnglandEURBill => b"BOE",
    Issuer::BankOfEnglandEURNote => b"BOEN",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum ListMethod {
    PreListedOnly,
    UserRequested,
}

define_enum_field_type!(NOT_REQUIRED, ListMethod, ListMethodFieldType {
    ListMethod::PreListedOnly => b"0",
    ListMethod::UserRequested => b"1",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum OptPayoutType {
    Vanilla,
    Capped,
    Binary
}

define_enum_field_type!(NOT_REQUIRED, OptPayoutType, OptPayoutTypeFieldType {
    OptPayoutType::Vanilla => b"1",
    OptPayoutType::Capped => b"2",
    OptPayoutType::Binary => b"3",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum OrdType {
    Market,
    Limit,
    StopOrStopLoss,
    StopLimit,
    MarketOnClose, //Deprecated in FIX 4.3
    WithOrWithout,
    LimitOrBetter, //Deprecated in FIX 4.4
    LimitWithOrWithout,
    OnBasis,
    OnClose, //Deprecated in FIX 4.3
    LimitOnClose, //Deprecated in FIX 4.3
    ForexMarket, //Deprecated in FIX 4.3
    PreviouslyQuoted,
    PreviouslyIndicated,
    ForexLimit, //Deprecated in FIX 4.3
    ForexSwap,
    ForexPreviouslyQuoted, //Deprecated in FIX 4.3
    Funari,
    MarketIfTouched,
    MarketWithLeftOverAsLimit,
    PreviousFundValuationPoint,
    NextFundValuationPoint,
    Pegged,
    CounterOrderSelection,
}

define_enum_field_type!(REQUIRED, OrdType, OrdTypeFieldType {
    OrdType::Market => b"1",
    OrdType::Limit => b"2",
    OrdType::StopOrStopLoss => b"3",
    OrdType::StopLimit => b"4",
    OrdType::MarketOnClose => b"5",
    OrdType::WithOrWithout => b"6",
    OrdType::LimitOrBetter => b"7",
    OrdType::LimitWithOrWithout => b"8",
    OrdType::OnBasis => b"9",
    OrdType::OnClose => b"A",
    OrdType::LimitOnClose => b"B",
    OrdType::ForexMarket => b"C",
    OrdType::PreviouslyQuoted => b"D",
    OrdType::PreviouslyIndicated => b"E",
    OrdType::ForexLimit => b"F",
    OrdType::ForexSwap => b"G",
    OrdType::ForexPreviouslyQuoted => b"H",
    OrdType::Funari => b"I",
    OrdType::MarketIfTouched => b"J",
    OrdType::MarketWithLeftOverAsLimit => b"K",
    OrdType::PreviousFundValuationPoint => b"L",
    OrdType::NextFundValuationPoint => b"M",
    OrdType::Pegged => b"P",
    OrdType::CounterOrderSelection => b"Q",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum PartyIDSource {
    BIC,
    GenerallyAcceptedMarketParticipantIdentifier,
    ProprietaryOrCustomCode,
    ISOCountryCode,
    SettlementEntityLocation,
    MIC,
    CSDParticipantOrMemberCode,
    UKNationalInsuranceOrPensionNumber,
    USSocialSecurityNumber,
    USEmployerOrTaxIDNumber,
    AustralianBusinessNumber,
    AustralianTaxFileNumber,
    KoreanInvestorID,
    TaiwaneseQualifiedForeignInvestorIDQFIIOrFID,
    TaiwaneseTradingAcct,
    MalaysianCentralDepositoryNumber,
    ChineseInvestorID,
    DirectedBroker,
}

define_enum_field_type!(REQUIRED, PartyIDSource, PartyIDSourceFieldType {
    PartyIDSource::BIC => b"B",
    PartyIDSource::GenerallyAcceptedMarketParticipantIdentifier => b"C",
    PartyIDSource::ProprietaryOrCustomCode => b"D",
    PartyIDSource::ISOCountryCode => b"E",
    PartyIDSource::SettlementEntityLocation => b"F",
    PartyIDSource::MIC => b"G",
    PartyIDSource::CSDParticipantOrMemberCode => b"H",
    PartyIDSource::UKNationalInsuranceOrPensionNumber => b"6",
    PartyIDSource::USSocialSecurityNumber => b"7",
    PartyIDSource::USEmployerOrTaxIDNumber => b"8",
    PartyIDSource::AustralianBusinessNumber => b"9",
    PartyIDSource::AustralianTaxFileNumber => b"A",
    PartyIDSource::KoreanInvestorID => b"1",
    PartyIDSource::TaiwaneseQualifiedForeignInvestorIDQFIIOrFID => b"2",
    PartyIDSource::TaiwaneseTradingAcct => b"3",
    PartyIDSource::MalaysianCentralDepositoryNumber => b"4",
    PartyIDSource::ChineseInvestorID => b"5",
    PartyIDSource::DirectedBroker => b"I",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum PartyRole {
    CentralRegistrationDepository,
    ClearingAccount,
    AcceptableSettlingCounterparty,
    UnacceptableSettlingCounterparty,
    ExecutingFirm,
    BrokerOfCredit,
    ClientID,
    ClearingFirm,
    InvestorID,
    IntroducingFirm,
    EnteringFirm,
    LocateOrLendingFirm,
    FundManagerClientID,
    SettlementLocation,
    OrderOriginationTrader,
    ExecutingTrader,
    OrderOriginationFirm,
    GiveupClearingFirm,
    CorrespondantClearingFirm,
    ExecutingSystem,
    ContraFirm,
    ContraClearingFirm,
    SponsoringFirm,
    UnderlyingContraFirm,
    ClearingOrganization,
    Exchange,
    CustomerAccount,
    CorrespondentClearingOrganization,
    CorrespondentBroker,
    BuyerOrSeller,
    Custodian,
    Intermediary,
    Agent,
    SubCustodian,
    Beneficiary,
    InterestedParty,
    RegulatoryBody,
    LiquidityProvider,
    EnteringTrader,
    ContraTrader,
    PositionAccount,
    ContraInvestorID,
    TransferToFirm,
    ContraPositionAccount,
    ContraExchange,
    InternalCarryAccount,
    OrderEntryOperatorID,
    SecondaryAccountNumber,
    ForeignFirm,
    ThirdPartyAllocationFirm,
    ClaimingAccount,
    AssetManager,
    PledgorAccount,
    PledgeeAccount,
    LargeTraderReportableAccount,
    TraderMnemonic,
    SenderLocation,
    SessionID,
    AcceptableCounterparty,
    UnacceptableCounterparty,
    EnteringUnit,
    ExecutingUnit,
    IntroducingBroker,
    QuoteOriginator,
    ReportOriginator,
    SystematicInternaliser,
    MultilateralTradingFacility,
    RegulatedMarket,
    MarketMaker,
    InvestmentFirm,
    HostCompetentAuthority,
    HomeCompetentAuthority,
    CompetentAuthorityOfTheMostRelevantMarketInTermsOfLiquidity,
    CompetentAuthorityOfTheTransactionVenue,
    ReportingIntermediary,
    ExecutionVenue,
    MarketDataEntryOriginator,
    LocationID,
    DeskID,
    MarketDataMarket,
    AllocationEntity,
    PrimeBrokerProvidingGeneralTradeServices,
    StepOutFirm,
    BrokerClearingID,
}

define_enum_field_type!(NOT_REQUIRED, PartyRole, PartyRoleFieldType {
    PartyRole::CentralRegistrationDepository => b"82",
    PartyRole::ClearingAccount => b"83",
    PartyRole::AcceptableSettlingCounterparty => b"84",
    PartyRole::UnacceptableSettlingCounterparty => b"85",
    PartyRole::ExecutingFirm => b"1",
    PartyRole::BrokerOfCredit => b"2",
    PartyRole::ClientID => b"3",
    PartyRole::ClearingFirm => b"4",
    PartyRole::InvestorID => b"5",
    PartyRole::IntroducingFirm => b"6",
    PartyRole::EnteringFirm => b"7",
    PartyRole::LocateOrLendingFirm => b"8",
    PartyRole::FundManagerClientID => b"9",
    PartyRole::SettlementLocation => b"10",
    PartyRole::OrderOriginationTrader => b"11",
    PartyRole::ExecutingTrader => b"12",
    PartyRole::OrderOriginationFirm => b"13",
    PartyRole::GiveupClearingFirm => b"14",
    PartyRole::CorrespondantClearingFirm => b"15",
    PartyRole::ExecutingSystem => b"16",
    PartyRole::ContraFirm => b"17",
    PartyRole::ContraClearingFirm => b"18",
    PartyRole::SponsoringFirm => b"19",
    PartyRole::UnderlyingContraFirm => b"20",
    PartyRole::ClearingOrganization => b"21",
    PartyRole::Exchange => b"22",
    PartyRole::CustomerAccount => b"24",
    PartyRole::CorrespondentClearingOrganization => b"25",
    PartyRole::CorrespondentBroker => b"26",
    PartyRole::BuyerOrSeller => b"27",
    PartyRole::Custodian => b"28",
    PartyRole::Intermediary => b"29",
    PartyRole::Agent => b"30",
    PartyRole::SubCustodian => b"31",
    PartyRole::Beneficiary => b"32",
    PartyRole::InterestedParty => b"33",
    PartyRole::RegulatoryBody => b"34",
    PartyRole::LiquidityProvider => b"35",
    PartyRole::EnteringTrader => b"36",
    PartyRole::ContraTrader => b"37",
    PartyRole::PositionAccount => b"38",
    PartyRole::ContraInvestorID => b"39",
    PartyRole::TransferToFirm => b"40",
    PartyRole::ContraPositionAccount => b"41",
    PartyRole::ContraExchange => b"42",
    PartyRole::InternalCarryAccount => b"43",
    PartyRole::OrderEntryOperatorID => b"44",
    PartyRole::SecondaryAccountNumber => b"45",
    PartyRole::ForeignFirm => b"46",
    PartyRole::ThirdPartyAllocationFirm => b"47",
    PartyRole::ClaimingAccount => b"48",
    PartyRole::AssetManager => b"49",
    PartyRole::PledgorAccount => b"50",
    PartyRole::PledgeeAccount => b"51",
    PartyRole::LargeTraderReportableAccount => b"52",
    PartyRole::TraderMnemonic => b"53",
    PartyRole::SenderLocation => b"54",
    PartyRole::SessionID => b"55",
    PartyRole::AcceptableCounterparty => b"56",
    PartyRole::UnacceptableCounterparty => b"57",
    PartyRole::EnteringUnit => b"58",
    PartyRole::ExecutingUnit => b"59",
    PartyRole::IntroducingBroker => b"60",
    PartyRole::QuoteOriginator => b"61",
    PartyRole::ReportOriginator => b"62",
    PartyRole::SystematicInternaliser => b"63",
    PartyRole::MultilateralTradingFacility => b"64",
    PartyRole::RegulatedMarket => b"65",
    PartyRole::MarketMaker => b"66",
    PartyRole::InvestmentFirm => b"67",
    PartyRole::HostCompetentAuthority => b"68",
    PartyRole::HomeCompetentAuthority => b"69",
    PartyRole::CompetentAuthorityOfTheMostRelevantMarketInTermsOfLiquidity => b"70",
    PartyRole::CompetentAuthorityOfTheTransactionVenue => b"71",
    PartyRole::ReportingIntermediary => b"72",
    PartyRole::ExecutionVenue => b"73",
    PartyRole::MarketDataEntryOriginator => b"74",
    PartyRole::LocationID => b"75",
    PartyRole::DeskID => b"76",
    PartyRole::MarketDataMarket => b"77",
    PartyRole::AllocationEntity => b"78",
    PartyRole::PrimeBrokerProvidingGeneralTradeServices => b"79",
    PartyRole::StepOutFirm => b"80",
    PartyRole::BrokerClearingID => b"81",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum PartySubIDType {
    Firm,
    Person,
    System,
    Application,
    FullLegalNameOfFirm,
    PostalAddress,
    PhoneNumber,
    EmailAddress,
    ContactName,
    SecuritiesAccountNumberForSettlementInstructions,
    RegistrationNumberForSettlementInstructionsAndConfirmations,
    RegisteredAddressForConfirmationPurposes,
    RegulatoryStatusForConfirmationPurposes,
    RegistrationNameForSettlementInstructions,
    CashAccountNumberForSettlementInstructions,
    BIC,
    CSDParticipantMemberCode,
    RegisteredAddress,
    FundAccountName,
    TelexNumber,
    FaxNumber,
    SecuritiesAccountName,
    CashAccountName,
    Department,
    LocationDesk,
    PositionAccountType,
    SecurityLocateID,
    MarketMaker,
    ElgibleCounterparty,
    ProfessionalClient,
    Location,
    ExecutionVenue,
    CurrencyDeliveryIdentifier,
    Reserved4000Plus(i64),
}

define_enum_field_type_with_reserved!(REQUIRED, PartySubIDType, PartySubIDTypeFieldType {
    PartySubIDType::Firm => 1,
    PartySubIDType::Person => 2,
    PartySubIDType::System => 3,
    PartySubIDType::Application => 4,
    PartySubIDType::FullLegalNameOfFirm => 5,
    PartySubIDType::PostalAddress => 6,
    PartySubIDType::PhoneNumber => 7,
    PartySubIDType::EmailAddress => 8,
    PartySubIDType::ContactName => 9,
    PartySubIDType::SecuritiesAccountNumberForSettlementInstructions => 10,
    PartySubIDType::RegistrationNumberForSettlementInstructionsAndConfirmations => 11,
    PartySubIDType::RegisteredAddressForConfirmationPurposes => 12,
    PartySubIDType::RegulatoryStatusForConfirmationPurposes => 13,
    PartySubIDType::RegistrationNameForSettlementInstructions => 14,
    PartySubIDType::CashAccountNumberForSettlementInstructions => 15,
    PartySubIDType::BIC => 16,
    PartySubIDType::CSDParticipantMemberCode => 17,
    PartySubIDType::RegisteredAddress => 18,
    PartySubIDType::FundAccountName => 19,
    PartySubIDType::TelexNumber => 20,
    PartySubIDType::FaxNumber => 21,
    PartySubIDType::SecuritiesAccountName => 22,
    PartySubIDType::CashAccountName => 23,
    PartySubIDType::Department => 24,
    PartySubIDType::LocationDesk => 25,
    PartySubIDType::PositionAccountType => 26,
    PartySubIDType::SecurityLocateID => 27,
    PartySubIDType::MarketMaker => 28,
    PartySubIDType::ElgibleCounterparty => 29,
    PartySubIDType::ProfessionalClient => 30,
    PartySubIDType::Location => 31,
    PartySubIDType::ExecutionVenue => 32,
    PartySubIDType::CurrencyDeliveryIdentifier => 33,
} PartySubIDType::Reserved4000Plus => WITH_MINIMUM 4000);

#[derive(Clone,PartialEq)]
pub enum PriceQuoteMethod {
    PercentOfPar,
    Standard,
    Index,
    InterestRateIndex,
}

define_enum_field_type!(NOT_REQUIRED, PriceQuoteMethod, PriceQuoteMethodFieldType {
    PriceQuoteMethod::PercentOfPar => b"PCTPAR",
    PriceQuoteMethod::Standard => b"STD",
    PriceQuoteMethod::Index => b"INDX",
    PriceQuoteMethod::InterestRateIndex => b"INT",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum Product {
    Agency,
    Commodity,
    Corporate,
    Currency,
    Equity,
    Government,
    Index,
    Loan,
    MoneyMarket,
    Mortgage,
    Municipal,
    Other,
    Financing,
}

define_enum_field_type!(NOT_REQUIRED, Product, ProductFieldType {
    Product::Agency => b"1",
    Product::Commodity => b"2",
    Product::Corporate => b"3",
    Product::Currency => b"4",
    Product::Equity => b"5",
    Product::Government => b"6",
    Product::Index => b"7",
    Product::Loan => b"8",
    Product::MoneyMarket => b"9",
    Product::Mortgage => b"10",
    Product::Municipal => b"11",
    Product::Other => b"12",
    Product::Financing => b"13",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum PutOrCall {
    Put,
    Call,
}

define_enum_field_type!(NOT_REQUIRED, PutOrCall, PutOrCallFieldType {
    PutOrCall::Put => b"0",
    PutOrCall::Call => b"1",
} MUST_BE_INT);

#[derive(Clone,Debug,PartialEq)]
pub enum RateSource {
    Bloomberg,
    Reuters,
    Telerate,
    Other,
}

define_enum_field_type!(REQUIRED, RateSource, RateSourceFieldType {
    RateSource::Bloomberg => b"0",
    RateSource::Reuters => b"1",
    RateSource::Telerate => b"2",
    RateSource::Other => b"99",
} MUST_BE_INT);

#[derive(Clone,Debug,PartialEq)]
pub enum RateSourceType {
    Primary,
    Secondary,
}

define_enum_field_type!(REQUIRED, RateSourceType, RateSourceTypeFieldType {
    RateSourceType::Primary => b"0",
    RateSourceType::Secondary => b"1",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum RestructuringType {
    FullRestructuring,
    ModifiedRestructuring,
    ModifiedModRestructuring,
    NoRestructuringSpecified,
}

define_enum_field_type!(NOT_REQUIRED, RestructuringType, RestructuringTypeFieldType {
    RestructuringType::FullRestructuring => b"FR",
    RestructuringType::ModifiedRestructuring => b"MR",
    RestructuringType::ModifiedModRestructuring => b"MM",
    RestructuringType::NoRestructuringSpecified => b"XR",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum RoutingType {
    TargetFirm,
    TargetList,
    BlockFirm,
    BlockList,
}

define_enum_field_type!(REQUIRED, RoutingType, RoutingTypeFieldType {
    RoutingType::TargetFirm => b"1",
    RoutingType::TargetList => b"2",
    RoutingType::BlockFirm => b"3",
    RoutingType::BlockList => b"4",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum SecurityIDSource {
    CUSIP,
    SEDOL,
    QUIK,
    ISINNumber,
    RICCode,
    ISOCurrencyCode,
    ISOCountryCode,
    ExchangeSymbol,
    ConsolidatedTapeAssociationSymbol,
    BloombergSymbol,
    Wertpapier,
    Dutch,
    Valoren,
    Sicovam,
    Belgian,
    Common,
    ClearingHouseOrClearingOrganization,
    ISDAOrFpMLProductSpecification,
    OptionPriceReportingAuthority,
    ISDAOrFpMLProductURL,
    LetterOfCredit,
    MarketplaceAssignedIdentifier,
    Other(Vec<u8>),
}

define_enum_field_type_with_reserved!(BYTES, SecurityIDSource, RequiredSecurityIDSourceFieldType, NotRequiredSecurityIDSourceFieldType {
    SecurityIDSource::CUSIP => b"1",
    SecurityIDSource::SEDOL => b"2",
    SecurityIDSource::QUIK => b"3",
    SecurityIDSource::ISINNumber => b"4",
    SecurityIDSource::RICCode => b"5",
    SecurityIDSource::ISOCurrencyCode => b"6",
    SecurityIDSource::ISOCountryCode => b"7",
    SecurityIDSource::ExchangeSymbol => b"8",
    SecurityIDSource::ConsolidatedTapeAssociationSymbol => b"9",
    SecurityIDSource::BloombergSymbol => b"A",
    SecurityIDSource::Wertpapier => b"B",
    SecurityIDSource::Dutch => b"C",
    SecurityIDSource::Valoren => b"D",
    SecurityIDSource::Sicovam => b"E",
    SecurityIDSource::Belgian => b"F",
    SecurityIDSource::Common => b"G",
    SecurityIDSource::ClearingHouseOrClearingOrganization => b"H",
    SecurityIDSource::ISDAOrFpMLProductSpecification => b"I",
    SecurityIDSource::OptionPriceReportingAuthority => b"J",
    SecurityIDSource::ISDAOrFpMLProductURL => b"K",
    SecurityIDSource::LetterOfCredit => b"L",
    SecurityIDSource::MarketplaceAssignedIdentifier => b"M",
} SecurityIDSource::Other);

#[derive(Clone,PartialEq)]
pub enum SecurityStatus {
    Active,
    Inactive,
}

define_enum_field_type!(REQUIRED, SecurityStatus, SecurityStatusFieldType {
    SecurityStatus::Active => b"1",
    SecurityStatus::Inactive => b"2",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum SecurityType {
    USTreasureNote, //Deprecated in FIX 4.4, use USTreasuryNote.
    USTreasureBill, //Deprecated in FIX 4.4, use USTreasuryBill.
    EuroSupranationalCoupons,
    FederalAgencyCoupon,
    FederalAgencyDiscountNote,
    PrivateExportFunding,
    USDSupranationalCoupons,
    CorporateBond,
    CorporatePrivatePlacement,
    ConvertibleBond,
    DualCurrency,
    EuroCorporateBond,
    EuroCorporateFloatingRateNotes,
    USCorporateFloatingRateNotes,
    IndexedLinked,
    StructuredNotes,
    YankeeCorporateBond,
    ForeignExchangeContract, //Deprecated in FIX 5.2 SP1
    NonDeliverableForward,
    FXSpot,
    FXForward,
    FXSwap,
    CreditDefaultSwap,
    Future,
    Option,
    OptionsOnFutures,
    OptionsOnPhysical,
    InterestRateSwap,
    OptionsOnCombo,
    CommonStock,
    PreferredStock,
    Repurchase,
    Forward,
    BuySellback,
    SecuritiesLoan,
    SecuritiesPledge,
    BradyBond,
    CanadianTreasuryNotes,
    CanadianTreasuryBills,
    EuroSovereigns,
    CanadianProvincialBonds,
    TreasuryBillNonUS,
    USTreasuryBond,
    InterestStripFromAnyBondOrNote,
    USTreasuryBill,
    TreasuryInflationProtectedSecurities,
    PrincipalStripOfACallableBondOrNote,
    PrincipalStripFromANonCallableBondOrNote,
    USTreasuryNote,
    TermLoan,
    RevolverLoan,
    RevolverOrTermLoan,
    BridgeLoan,
    LetterOfCredit,
    SwingLineFacility,
    DebtorInPossession,
    Defaulted,
    Withdrawn,
    Replaced,
    Matured,
    AmendedAndRestated,
    Retired,
    BankersAcceptance,
    BankDepositoryNote,
    BankNotes,
    BillOfExchanges,
    CanadianMoneyMarkets,
    CertificateOfDeposit,
    CallLoans,
    CommercialPaper,
    DepositNotes,
    EuroCertificateOfDeposit,
    EuroCommercialPaper,
    LiquidityNote,
    MediumTermNotes,
    Overnight,
    PromissoryNote,
    ShortTermLoanNote,
    PlazosFijos,
    SecuredLiquidityNote,
    TimeDeposit,
    TermLiquidityNote,
    ExtendedCommNote,
    YankeeCertificateOfDeposit,
    AssetBackedSecurities,
    CanadianMortgageBonds,
    CorpMortgageBackedSecurities,
    CollateralizedMortgageObligation,
    IOETTEMortgage,
    MortgageBackedSecurities,
    MortgageInterestOnly,
    MortgagePrincipalOnly,
    MortgagePrivatePlacement,
    MiscellaneousPassThrough,
    Pfandbriefe,
    ToBeAnnounced,
    OtherAnticipationNotes,
    CertificateOfObligation,
    CertificateOfParticipation,
    GeneralObligationBonds,
    MandatoryTender,
    RevenueAnticipationNote,
    RevenueBonds,
    SpecialAssessment,
    SpecialObligation,
    SpecialTax,
    TaxAnticipationNote,
    TaxAllocation,
    TaxExemptCommercialPaper,
    TaxableMunicipalCP,
    TaxRevenueAnticipationNote,
    VariableRateDemandNote,
    Warrant,
    MutualFund,
    MultilegInstrument,
    NoSecurityType,
    Wildcard,
    Cash,
    Other(Vec<u8>),
}

define_enum_field_type_with_reserved!(BYTES, SecurityType, RequiredSecurityTypeFieldType, NotRequiredSecurityTypeFieldType {
    SecurityType::USTreasureNote => b"UST",
    SecurityType::USTreasureBill => b"USTB",
    SecurityType::EuroSupranationalCoupons => b"EUSUPRA",
    SecurityType::FederalAgencyCoupon => b"FAC",
    SecurityType::FederalAgencyDiscountNote => b"FADN",
    SecurityType::PrivateExportFunding => b"PEF",
    SecurityType::USDSupranationalCoupons => b"SUPRA",
    SecurityType::CorporateBond => b"CORP",
    SecurityType::CorporatePrivatePlacement => b"CPP",
    SecurityType::ConvertibleBond => b"CB",
    SecurityType::DualCurrency => b"DUAL",
    SecurityType::EuroCorporateBond => b"EUCORP",
    SecurityType::EuroCorporateFloatingRateNotes => b"EUFRN",
    SecurityType::USCorporateFloatingRateNotes => b"FRN",
    SecurityType::IndexedLinked => b"XLINKD",
    SecurityType::StructuredNotes => b"STRUCT",
    SecurityType::YankeeCorporateBond => b"YANK",
    SecurityType::ForeignExchangeContract => b"FOR",
    SecurityType::NonDeliverableForward => b"FXNDF",
    SecurityType::FXSpot => b"FXSPOT",
    SecurityType::FXForward => b"FXFWD",
    SecurityType::FXSwap => b"FXSWAP",
    SecurityType::CreditDefaultSwap => b"CDS",
    SecurityType::Future => b"FUT",
    SecurityType::Option => b"OPT",
    SecurityType::OptionsOnFutures => b"OOF",
    SecurityType::OptionsOnPhysical => b"OOP",
    SecurityType::InterestRateSwap => b"IRS",
    SecurityType::OptionsOnCombo => b"OOC",
    SecurityType::CommonStock => b"CS",
    SecurityType::PreferredStock => b"PS",
    SecurityType::Repurchase => b"REPO",
    SecurityType::Forward => b"FORWARD",
    SecurityType::BuySellback => b"BUYSELL",
    SecurityType::SecuritiesLoan => b"SECLOAN",
    SecurityType::SecuritiesPledge => b"SECPLEDGE",
    SecurityType::BradyBond => b"BRADY",
    SecurityType::CanadianTreasuryNotes => b"CAN",
    SecurityType::CanadianTreasuryBills => b"CTB",
    SecurityType::EuroSovereigns => b"EUSOV",
    SecurityType::CanadianProvincialBonds => b"PROV",
    SecurityType::TreasuryBillNonUS => b"TB",
    SecurityType::USTreasuryBond => b"TBOND",
    SecurityType::InterestStripFromAnyBondOrNote => b"TINT",
    SecurityType::USTreasuryBill => b"TBILL",
    SecurityType::TreasuryInflationProtectedSecurities => b"TIPS",
    SecurityType::PrincipalStripOfACallableBondOrNote => b"TCAL",
    SecurityType::PrincipalStripFromANonCallableBondOrNote => b"TPRN",
    SecurityType::USTreasuryNote => b"TNOTE",
    SecurityType::TermLoan => b"TERM",
    SecurityType::RevolverLoan => b"RVLV",
    SecurityType::RevolverOrTermLoan => b"RVLTRM",
    SecurityType::BridgeLoan => b"BRIDGE",
    SecurityType::LetterOfCredit => b"LOFC",
    SecurityType::SwingLineFacility => b"SWING",
    SecurityType::DebtorInPossession => b"DINP",
    SecurityType::Defaulted => b"DEFLTED",
    SecurityType::Withdrawn => b"WITHDRN",
    SecurityType::Replaced => b"REPLACD",
    SecurityType::Matured => b"MATURED",
    SecurityType::AmendedAndRestated => b"AMENDED",
    SecurityType::Retired => b"RETIRED",
    SecurityType::BankersAcceptance => b"BA",
    SecurityType::BankDepositoryNote => b"BDN",
    SecurityType::BankNotes => b"BN",
    SecurityType::BillOfExchanges => b"BOX",
    SecurityType::CanadianMoneyMarkets => b"CAMM",
    SecurityType::CertificateOfDeposit => b"CD",
    SecurityType::CallLoans => b"CL",
    SecurityType::CommercialPaper => b"CP",
    SecurityType::DepositNotes => b"DN",
    SecurityType::EuroCertificateOfDeposit => b"EUCD",
    SecurityType::EuroCommercialPaper => b"EUCP",
    SecurityType::LiquidityNote => b"LQN",
    SecurityType::MediumTermNotes => b"MTN",
    SecurityType::Overnight => b"ONITE",
    SecurityType::PromissoryNote => b"PN",
    SecurityType::ShortTermLoanNote => b"STN",
    SecurityType::PlazosFijos => b"PZFJ",
    SecurityType::SecuredLiquidityNote => b"SLQN",
    SecurityType::TimeDeposit => b"TD",
    SecurityType::TermLiquidityNote => b"TLQN",
    SecurityType::ExtendedCommNote => b"XCN",
    SecurityType::YankeeCertificateOfDeposit => b"YCD",
    SecurityType::AssetBackedSecurities => b"ABS",
    SecurityType::CanadianMortgageBonds => b"CMB",
    SecurityType::CorpMortgageBackedSecurities => b"CMBS",
    SecurityType::CollateralizedMortgageObligation => b"CMO",
    SecurityType::IOETTEMortgage => b"IET",
    SecurityType::MortgageBackedSecurities => b"MBS",
    SecurityType::MortgageInterestOnly => b"MIO",
    SecurityType::MortgagePrincipalOnly => b"MPO",
    SecurityType::MortgagePrivatePlacement => b"MPP",
    SecurityType::MiscellaneousPassThrough => b"MPT",
    SecurityType::Pfandbriefe => b"PFAND",
    SecurityType::ToBeAnnounced => b"TBA",
    SecurityType::OtherAnticipationNotes => b"AN",
    SecurityType::CertificateOfObligation => b"COFO",
    SecurityType::CertificateOfParticipation => b"COFP",
    SecurityType::GeneralObligationBonds => b"GO",
    SecurityType::MandatoryTender => b"MT",
    SecurityType::RevenueAnticipationNote => b"RAN",
    SecurityType::RevenueBonds => b"REV",
    SecurityType::SpecialAssessment => b"SPCLA",
    SecurityType::SpecialObligation => b"SPCLO",
    SecurityType::SpecialTax => b"SPCLT",
    SecurityType::TaxAnticipationNote => b"TAN",
    SecurityType::TaxAllocation => b"TAXA",
    SecurityType::TaxExemptCommercialPaper => b"TECP",
    SecurityType::TaxableMunicipalCP => b"TMCP",
    SecurityType::TaxRevenueAnticipationNote => b"TRAN",
    SecurityType::VariableRateDemandNote => b"VRDN",
    SecurityType::Warrant => b"WAR",
    SecurityType::MutualFund => b"MF",
    SecurityType::MultilegInstrument => b"MLEG",
    SecurityType::NoSecurityType => b"NONE",
    SecurityType::Wildcard => b"?",
    SecurityType::Cash => b"CASH",
} SecurityType::Other);

#[derive(Clone,PartialEq)]
pub enum Seniority {
    SeniorSecured,
    Senior,
    Subordinated,
}

define_enum_field_type!(NOT_REQUIRED, Seniority, SeniorityFieldType {
    Seniority::SeniorSecured => b"SD",
    Seniority::Senior => b"SR",
    Seniority::Subordinated => b"SB",
} MUST_BE_STRING);

#[derive(Clone,Debug,PartialEq)]
pub enum SessionRejectReason {
    InvalidTagNumber,
    RequiredTagMissing,
    TagNotDefinedForThisMessageType,
    UndefinedTag,
    TagSpecifiedWithoutAValue,
    ValueIsIncorrectForThisTag,
    IncorrectDataFormatForValue,
    DecryptionProblem,
    SignatureProblem,
    CompIDProblem,
    SendingTimeAccuracyProblem,
    InvalidMsgType,
    XMLValidationError,
    TagAppearsMoreThanOnce,
    TagSpecifiedOutOfRequiredOrder,
    RepeatingGroupFieldsOutOfOrder,
    IncorrectNumInGroupCountForRepeatingGroup,
    NonDataValueIncludesFieldDelimiter,
    InvalidOrUnsupportedApplicationVersion,
    Other,
    Reserved100Plus(i64),
}

define_enum_field_type_with_reserved!(NOT_REQUIRED, SessionRejectReason, SessionRejectReasonFieldType {
    SessionRejectReason::InvalidTagNumber => 0,
    SessionRejectReason::RequiredTagMissing => 1,
    SessionRejectReason::TagNotDefinedForThisMessageType => 2,
    SessionRejectReason::UndefinedTag => 3,
    SessionRejectReason::TagSpecifiedWithoutAValue => 4,
    SessionRejectReason::ValueIsIncorrectForThisTag => 5,
    SessionRejectReason::IncorrectDataFormatForValue => 6,
    SessionRejectReason::DecryptionProblem => 7,
    SessionRejectReason::SignatureProblem => 8,
    SessionRejectReason::CompIDProblem => 9,
    SessionRejectReason::SendingTimeAccuracyProblem => 10,
    SessionRejectReason::InvalidMsgType => 11,
    SessionRejectReason::XMLValidationError => 12,
    SessionRejectReason::TagAppearsMoreThanOnce => 13,
    SessionRejectReason::TagSpecifiedOutOfRequiredOrder => 14,
    SessionRejectReason::RepeatingGroupFieldsOutOfOrder => 15,
    SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup => 16,
    SessionRejectReason::NonDataValueIncludesFieldDelimiter => 17,
    SessionRejectReason::InvalidOrUnsupportedApplicationVersion => 18,
    SessionRejectReason::Other => 99,
} SessionRejectReason::Reserved100Plus => WITH_MINIMUM 100);

#[derive(Clone,PartialEq)]
pub enum SettlMethod {
    CashSettlementRequired,
    PhysicalSettlementRequired,
}

define_enum_field_type!(NOT_REQUIRED, SettlMethod, SettlMethodFieldType {
    SettlMethod::CashSettlementRequired => b"C",
    SettlMethod::PhysicalSettlementRequired => b"P",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum SettlType {
    RegularOrFXSpotSettlement,
    Cash,
    NextDay,
    TPlus2,
    TPlus3,
    TPlus4,
    Future,
    WhenAndIfIssued,
    SellersOption,
    TPlus5,
    BrokenDate,
    FXSpotNextSettlement,
    Days(u64),
    Months(u64),
    Weeks(u64),
    Years(u64),
}

impl SettlType {
    fn new(bytes: &[u8]) -> Option<SettlType> {
        Some(match bytes {
            b"0" => SettlType::RegularOrFXSpotSettlement,
            b"1" => SettlType::Cash,
            b"2" => SettlType::NextDay,
            b"3" => SettlType::TPlus2,
            b"4" => SettlType::TPlus3,
            b"5" => SettlType::TPlus4,
            b"6" => SettlType::Future,
            b"7" => SettlType::WhenAndIfIssued,
            b"8" => SettlType::SellersOption,
            b"9" => SettlType::TPlus5,
            b"B" => SettlType::BrokenDate,
            b"C" => SettlType::FXSpotNextSettlement,
            _ if bytes.len() > 1 => {
                let number_string = String::from_utf8_lossy(&bytes[1..]).into_owned();
                let number = match u64::from_str(&number_string) {
                    Err(_) => return None,
                    Ok(number) if number == 0 => {
                        //All of the following tenors require an integer > 0.
                        return None;
                    },
                    Ok(number) => number,
                };

                match bytes[0] {
                    b'D' => SettlType::Days(number),
                    b'M' => SettlType::Months(number),
                    b'W' => SettlType::Weeks(number),
                    b'Y' => SettlType::Years(number),
                    _ => return None,
                }
            },
            _ => return None,
        })
    }

    fn read(&self,buf: &mut Vec<u8>) -> usize {
        match *self {
            SettlType::RegularOrFXSpotSettlement => buf.write(b"0").unwrap(),
            SettlType::Cash => buf.write(b"1").unwrap(),
            SettlType::NextDay => buf.write(b"2").unwrap(),
            SettlType::TPlus2 => buf.write(b"3").unwrap(),
            SettlType::TPlus3 => buf.write(b"4").unwrap(),
            SettlType::TPlus4 => buf.write(b"5").unwrap(),
            SettlType::Future => buf.write(b"6").unwrap(),
            SettlType::WhenAndIfIssued => buf.write(b"7").unwrap(),
            SettlType::SellersOption => buf.write(b"8").unwrap(),
            SettlType::TPlus5 => buf.write(b"9").unwrap(),
            SettlType::BrokenDate => buf.write(b"B").unwrap(),
            SettlType::FXSpotNextSettlement => buf.write(b"C").unwrap(),
            SettlType::Days(number) => {
                buf.write(b"D").unwrap() +
                buf.write(number.to_string().as_bytes()).unwrap()
            },
            SettlType::Months(number) => {
                buf.write(b"M").unwrap() +
                buf.write(number.to_string().as_bytes()).unwrap()
            },
            SettlType::Weeks(number) => {
                buf.write(b"W").unwrap() +
                buf.write(number.to_string().as_bytes()).unwrap()
            },
            SettlType::Years(number) => {
                buf.write(b"Y").unwrap() +
                buf.write(number.to_string().as_bytes()).unwrap()
            },
        }
    }
}

pub struct SettlTypeFieldType;

impl FieldType for SettlTypeFieldType {
    type Type = Option<SettlType>;

    fn default_value() -> Self::Type {
        None
    }

    fn set_value(field: &mut Self::Type,bytes: &[u8]) -> Result<(),SetValueError> {
        if let Some(value) = SettlType::new(bytes) {
            *field = Some(value);
            return Ok(())
        }

        Err(SetValueError::OutOfRange)
    }

    fn is_empty(field: &Self::Type) -> bool {
        field.is_none()
    }

    fn len(_field: &Self::Type) -> usize {
        0 //Unused for this type.
    }

    fn read(field: &Self::Type,buf: &mut Vec<u8>) -> usize {
        if let Some(ref field) = *field {
            return field.read(buf)
        }

        0
    }
}

#[derive(Clone,PartialEq)]
pub enum Side {
    Buy,
    Sell,
    BuyMinus,
    SellPlus,
    SellShort,
    SellShortExempt,
    Undisclosed,
    Cross,
    CrossShort,
    CrossShortExempt,
    AsDefined,
    Opposite,
    Subscribe,
    Redeem,
    Lend,
    Borrow,
}

define_enum_field_type!(REQUIRED_AND_NOT_REQUIRED, Side, RequiredSideFieldType, NotRequiredSideFieldType {
    Side::Buy => b"1",
    Side::Sell => b"2",
    Side::BuyMinus => b"3",
    Side::SellPlus => b"4",
    Side::SellShort => b"5",
    Side::SellShortExempt => b"6",
    Side::Undisclosed => b"7",
    Side::Cross => b"8",
    Side::CrossShort => b"9",
    Side::CrossShortExempt => b"A",
    Side::AsDefined => b"B",
    Side::Opposite => b"C",
    Side::Subscribe => b"D",
    Side::Redeem => b"E",
    Side::Lend => b"F",
    Side::Borrow => b"G",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum StipulationType {
    AlternativeMinimumTax,
    AutoReinvestment,
    BankQualified,
    BargainConditions,
    CouponRange,
    ISOCurrencyCode,
    CustomStartOrEndDate,
    Geographics,
    ValuationDiscount,
    Insured,
    YearOrYearAndMonthOfIssue,
    IssuersTicker,
    IssueSizeRange,
    LookbackDays,
    ExplicitLotIdentifier,
    LotVariance,
    MaturityYearAndMonth,
    MaturityRange,
    MaximumSubstitutions,
    MinimumDenomination,
    MinimumIncrement,
    MinimumQuantity,
    PaymentFrequency,
    NumberOfPieces,
    PoolsMaximum,
    PoolsPerLot,
    PoolsPerMillion,
    PoolsPerTrade,
    PriceRange,
    PricingFrequency,
    ProductionYear,
    CallProtection,
    Purpose,
    BenchmarkPriceSource,
    RatingSourceAndRange,
    TypeOfRedemption,
    Restricted,
    MarketSector,
    SecurityType,
    Structure,
    SubstitutionsFrequency,
    SubstitutionsLeft,
    FreeformText,
    TradeVariance,
    WeightedAverageCoupon,
    WeightedAverageLifeCoupon,
    WeightedAverageLoanAge,
    WeightedAverageMaturity,
    WholePool,
    YieldRange,
    AverageFICOScore,
    AverageLoanSize,
    MaximumLoanBalance,
    PoolIdentifier,
    TypeOfRollTrade,
    ReferenceToRollingOrClosingTrade,
    PrincipalOfRollingOrClosingTrade,
    InterestOfRollingOrClosingTrade,
    AvailableOfferQuantityToBeShownToTheStreet,
    BrokersSalesCredit,
    OfferPriceToBeShownToInternalBrokers,
    OfferQuantityToBeShownToInternalBrokers,
    MinimumResidualOfferQuantity,
    MaximumOrderSize,
    OrderQuantityIncrement,
    PrimaryOrSecondaryMarketIndicator,
    BrokerSalesCreditOverride,
    TradersCredit,
    DiscountRate,
    YieldToMaturity,
    AbsolutePrepaymentSpeed,
    ConstantPrepaymentPenalty,
    ConstantPrepaymentRate,
    ConstantPrepaymentYield,
    FinalCPROfHomeEquityPrepaymentCurve,
    PercentOfManufacturedHousingPrepaymentCurve,
    MonthlyPrepaymentRate,
    PercentOfProspectusPrepaymentCurve,
    PercentOfBMAPrepaymentCurve,
    SingleMonthlyMortality,
    Other(Vec<u8>),
}

define_enum_field_type_with_reserved!(BYTES, StipulationType, RequiredStipulationTypeFieldType, NotRequiredStipulationTypeFieldType {
    StipulationType::AlternativeMinimumTax => b"AMT",
    StipulationType::AutoReinvestment => b"AUTOREINV",
    StipulationType::BankQualified => b"BANKQUAL",
    StipulationType::BargainConditions => b"BGNCON",
    StipulationType::CouponRange => b"COUPON",
    StipulationType::ISOCurrencyCode => b"CURRENCY",
    StipulationType::CustomStartOrEndDate => b"CUSTOMDATE",
    StipulationType::Geographics => b"GEOG",
    StipulationType::ValuationDiscount => b"HAIRCUT",
    StipulationType::Insured => b"ISSURED",
    StipulationType::YearOrYearAndMonthOfIssue => b"ISSUE",
    StipulationType::IssuersTicker => b"ISSUER",
    StipulationType::IssueSizeRange => b"ISSUESIZE",
    StipulationType::LookbackDays => b"LOOKBACK",
    StipulationType::ExplicitLotIdentifier => b"LOT",
    StipulationType::LotVariance => b"LOTVAR",
    StipulationType::MaturityYearAndMonth => b"MAT",
    StipulationType::MaturityRange => b"MATURITY",
    StipulationType::MaximumSubstitutions => b"MAXSUBS",
    StipulationType::MinimumDenomination => b"MINDNOM",
    StipulationType::MinimumIncrement => b"MININCR",
    StipulationType::MinimumQuantity => b"MINQTY",
    StipulationType::PaymentFrequency => b"PAYFREQ",
    StipulationType::NumberOfPieces => b"PIECES",
    StipulationType::PoolsMaximum => b"PMAX",
    StipulationType::PoolsPerLot => b"PPL",
    StipulationType::PoolsPerMillion => b"PPM",
    StipulationType::PoolsPerTrade => b"PPT",
    StipulationType::PriceRange => b"PRICE",
    StipulationType::PricingFrequency => b"PRICEFREQ",
    StipulationType::ProductionYear => b"PROD",
    StipulationType::CallProtection => b"PROTECT",
    StipulationType::Purpose => b"PURPOSE",
    StipulationType::BenchmarkPriceSource => b"PXSOURCE",
    StipulationType::RatingSourceAndRange => b"RATING",
    StipulationType::TypeOfRedemption => b"REDEMPTION",
    StipulationType::Restricted => b"RESTRICTED",
    StipulationType::MarketSector => b"SECTOR",
    StipulationType::SecurityType => b"SECTYPE",
    StipulationType::Structure => b"STRUCT",
    StipulationType::SubstitutionsFrequency => b"SUBSFREQ",
    StipulationType::SubstitutionsLeft => b"SUBSLEFT",
    StipulationType::FreeformText => b"TEXT",
    StipulationType::TradeVariance => b"TRDVAR",
    StipulationType::WeightedAverageCoupon => b"WAC",
    StipulationType::WeightedAverageLifeCoupon => b"WAL",
    StipulationType::WeightedAverageLoanAge => b"WALA",
    StipulationType::WeightedAverageMaturity => b"WAM",
    StipulationType::WholePool => b"WHOLE",
    StipulationType::YieldRange => b"YIELD",
    StipulationType::AverageFICOScore => b"AVFICO",
    StipulationType::AverageLoanSize => b"AVSIZE",
    StipulationType::MaximumLoanBalance => b"MAXBAL",
    StipulationType::PoolIdentifier => b"POOL",
    StipulationType::TypeOfRollTrade => b"ROLLTYPE",
    StipulationType::ReferenceToRollingOrClosingTrade => b"REFTRADE",
    StipulationType::PrincipalOfRollingOrClosingTrade => b"REFPRIN",
    StipulationType::InterestOfRollingOrClosingTrade => b"REFINT",
    StipulationType::AvailableOfferQuantityToBeShownToTheStreet => b"AVAILQTY",
    StipulationType::BrokersSalesCredit => b"BROKERCREDIT",
    StipulationType::OfferPriceToBeShownToInternalBrokers => b"INTERNALPX",
    StipulationType::OfferQuantityToBeShownToInternalBrokers => b"INTERNALQTY",
    StipulationType::MinimumResidualOfferQuantity => b"LEAVEQTY",
    StipulationType::MaximumOrderSize => b"MAXORDQTY",
    StipulationType::OrderQuantityIncrement => b"ORDRINCR",
    StipulationType::PrimaryOrSecondaryMarketIndicator => b"PRIMARY",
    StipulationType::BrokerSalesCreditOverride => b"SALESCREDITOVR",
    StipulationType::TradersCredit => b"TRADERCREDIT",
    StipulationType::DiscountRate => b"DISCOUNT",
    StipulationType::YieldToMaturity => b"YTM",
    StipulationType::AbsolutePrepaymentSpeed => b"ABS",
    StipulationType::ConstantPrepaymentPenalty => b"CPP",
    StipulationType::ConstantPrepaymentRate => b"CPR",
    StipulationType::ConstantPrepaymentYield => b"CPY",
    StipulationType::FinalCPROfHomeEquityPrepaymentCurve => b"HEP",
    StipulationType::PercentOfManufacturedHousingPrepaymentCurve => b"MHP",
    StipulationType::MonthlyPrepaymentRate => b"MPR",
    StipulationType::PercentOfProspectusPrepaymentCurve => b"PPC",
    StipulationType::PercentOfBMAPrepaymentCurve => b"PSA",
    StipulationType::SingleMonthlyMortality => b"SMM",
} StipulationType::Other);

#[derive(Clone,PartialEq)]
pub enum StrikePriceBoundaryMethod {
    LessThanUnderlyingPriceIsInTheMoney,
    LessThanOrEqualToTheUnderlyingPriceIsInTheMoney,
    EqualToTheUnderlyingPriceIsInTheMoney,
    GreaterThanOrEqualToUnderlyingPriceIsInTheMoney,
    GreaterThanUnderlyingIsInTheMoney,
}

define_enum_field_type!(NOT_REQUIRED, StrikePriceBoundaryMethod, StrikePriceBoundaryMethodFieldType {
    StrikePriceBoundaryMethod::LessThanUnderlyingPriceIsInTheMoney => b"1",
    StrikePriceBoundaryMethod::LessThanOrEqualToTheUnderlyingPriceIsInTheMoney => b"2",
    StrikePriceBoundaryMethod::EqualToTheUnderlyingPriceIsInTheMoney => b"3",
    StrikePriceBoundaryMethod::GreaterThanOrEqualToUnderlyingPriceIsInTheMoney => b"4",
    StrikePriceBoundaryMethod::GreaterThanUnderlyingIsInTheMoney => b"5",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum StrikePriceDeterminationMethod {
    FixedStrike,
    StrikeSetAtExpirationToUnderlyingOrOtherValue,
    StrikeSetToAverageOfUnderlyingSettlementPriceAcrossTheLifeOfTheOption,
    StrikeSetToOptimalValue,
    Reserved100Plus(i64),
}

define_enum_field_type_with_reserved!(NOT_REQUIRED, StrikePriceDeterminationMethod, StrikePriceDeterminationMethodFieldType {
    StrikePriceDeterminationMethod::FixedStrike => 1,
    StrikePriceDeterminationMethod::StrikeSetAtExpirationToUnderlyingOrOtherValue => 2,
    StrikePriceDeterminationMethod::StrikeSetToAverageOfUnderlyingSettlementPriceAcrossTheLifeOfTheOption => 3,
    StrikePriceDeterminationMethod::StrikeSetToOptimalValue => 4,
} StrikePriceDeterminationMethod::Reserved100Plus => WITH_MINIMUM 100);

#[derive(Clone,PartialEq)]
pub enum SymbolSfx {
    EUCPWithLumpSumInterestRatherThanDiscountPrice,
    WhenIssued,
    Other(Vec<u8>),
}

define_enum_field_type_with_reserved!(BYTES, SymbolSfx, RequiredSymbolSfxFieldType, NotRequiredSymbolSfxFieldType {
    SymbolSfx::EUCPWithLumpSumInterestRatherThanDiscountPrice => b"CD",
    SymbolSfx::WhenIssued => b"WI",
} SymbolSfx::Other);

#[derive(Clone,PartialEq)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    AtTheOpening,
    ImmediateOrCancel,
    FillOrKill,
    GoodTillCrossing,
    GoodTillDate,
    AtTheClose,
    GoodThroughCrossing,
    AtCrossing,
}

define_enum_field_type!(NOT_REQUIRED, TimeInForce, TimeInForceFieldType {
    TimeInForce::Day => b"0",
    TimeInForce::GoodTillCancel => b"1",
    TimeInForce::AtTheOpening => b"2",
    TimeInForce::ImmediateOrCancel => b"3",
    TimeInForce::FillOrKill => b"4",
    TimeInForce::GoodTillCrossing => b"5",
    TimeInForce::GoodTillDate => b"6",
    TimeInForce::AtTheClose => b"7",
    TimeInForce::GoodThroughCrossing => b"8",
    TimeInForce::AtCrossing => b"9",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum TimeUnit {
    Hour,
    Minute,
    Second,
    Day,
    Week,
    Month,
    Year,
    Other(Vec<u8>),
}

define_enum_field_type_with_reserved!(BYTES, TimeUnit, RequiredTimeUnitFieldType, NotRequiredTimeUnitFieldType {
    TimeUnit::Hour => b"H",
    TimeUnit::Minute => b"Min",
    TimeUnit::Second => b"S",
    TimeUnit::Day => b"D",
    TimeUnit::Week => b"Wk",
    TimeUnit::Month => b"Mo",
    TimeUnit::Year => b"Yr",
} TimeUnit::Other);

#[derive(Clone,PartialEq)]
pub enum UnderlyingCashType {
    Fixed,
    Diff,
}

define_enum_field_type!(NOT_REQUIRED, UnderlyingCashType, UnderlyingCashTypeFieldType {
    UnderlyingCashType::Fixed => b"FIXED",
    UnderlyingCashType::Diff => b"DIFF",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum UnderlyingFXRateCalc {
    Divide,
    Multiply,
}

define_enum_field_type!(NOT_REQUIRED, UnderlyingFXRateCalc, UnderlyingFXRateCalcFieldType {
    UnderlyingFXRateCalc::Divide => b"D",
    UnderlyingFXRateCalc::Multiply => b"M",
} MUST_BE_CHAR);

#[derive(Clone,PartialEq)]
pub enum UnderlyingPriceDeterminationMethod {
    Regular,
    SpecialReference,
    OptimalValue,
    AverageValue,
}

define_enum_field_type!(NOT_REQUIRED, UnderlyingPriceDeterminationMethod, UnderlyingPriceDeterminationMethodFieldType {
    UnderlyingPriceDeterminationMethod::Regular => b"1",
    UnderlyingPriceDeterminationMethod::SpecialReference => b"2",
    UnderlyingPriceDeterminationMethod::OptimalValue => b"3",
    UnderlyingPriceDeterminationMethod::AverageValue => b"4",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum UnderlyingSettlementType {
    TPlus1,
    TPlus3,
    TPlus4,
}

define_enum_field_type!(NOT_REQUIRED, UnderlyingSettlementType, UnderlyingSettlementTypeFieldType {
    UnderlyingSettlementType::TPlus1 => b"2",
    UnderlyingSettlementType::TPlus3 => b"4",
    UnderlyingSettlementType::TPlus4 => b"5",
} MUST_BE_INT);

#[derive(Clone,PartialEq)]
pub enum UnitOfMeasure {
    BillionCubicFeet,
    MillionBarrels, //Deprecated in FIX 5.0 SP1.
    OneMillionBTU,
    MegawattHours,
    Barrels,
    Bushels,
    Pounds,
    Gallons,
    TroyOunces,
    MetricTons, //Tonne
    Tons, //US
    USDollars,
    Allowances,
}

define_enum_field_type!(NOT_REQUIRED, UnitOfMeasure, UnitOfMeasureFieldType {
    UnitOfMeasure::BillionCubicFeet => b"Bcf",
    UnitOfMeasure::MillionBarrels => b"MMbbl",
    UnitOfMeasure::OneMillionBTU => b"MMBtu",
    UnitOfMeasure::MegawattHours => b"MWh",
    UnitOfMeasure::Barrels => b"Bbl",
    UnitOfMeasure::Bushels => b"Bu",
    UnitOfMeasure::Pounds => b"lbs",
    UnitOfMeasure::Gallons => b"Gal",
    UnitOfMeasure::TroyOunces => b"oz_tr",
    UnitOfMeasure::MetricTons => b"t",
    UnitOfMeasure::Tons => b"tn",
    UnitOfMeasure::USDollars => b"USD",
    UnitOfMeasure::Allowances => b"Alw",
} MUST_BE_STRING);

#[derive(Clone,PartialEq)]
pub enum ValuationMethod {
    CDSStyleCollateralizationOfMarketToMarketAndCoupon,
    CDSInDelivery,
    PremiumStyle,
    FuturesStyleMarkToMarket,
    FuturesStyleWithAnAttachedCashAdjustment,
}

define_enum_field_type!(NOT_REQUIRED, ValuationMethod, ValuationMethodFieldType {
    ValuationMethod::CDSStyleCollateralizationOfMarketToMarketAndCoupon => b"CDS",
    ValuationMethod::CDSInDelivery => b"CDSD",
    ValuationMethod::PremiumStyle => b"EQTY",
    ValuationMethod::FuturesStyleMarkToMarket => b"FUT",
    ValuationMethod::FuturesStyleWithAnAttachedCashAdjustment => b"FUTDA",
} MUST_BE_STRING);