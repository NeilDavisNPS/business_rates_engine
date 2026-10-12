use rust_decimal::Decimal;
use crate::models::calculation::{LiabilityCalculationRequest, AnnualBillingSegment, QualifyingReliefFlags};
use crate::models::enums::{Jurisdiction, OccupancyType, PartyType};
use crate::i18n::Language;

pub fn generate_narrative(
    req: &LiabilityCalculationRequest,
    segments: &[AnnualBillingSegment],
    total_net: Decimal,
    lang: Language,
) -> String {
    let mut lines = Vec::new();

    let (start_d, end_d) = req.resolve_overall_period();

    if lang.is_welsh() {
        lines.push("ADRODDIAD STATUDOL AR GYFRIFO TRETHI BUSNES & LLWYBR ARCHWILIO".to_string());
        lines.push("=============================================".to_string());
        let framework = match req.hereditament.jurisdiction {
            Jurisdiction::ENGLAND => "Deddf Cyllid Llywodraeth Leol 1988 (fel y'i diwygiwyd gan Ddeddf Ardrethi Annomestig 2023) ar gyfer Lloegr",
            Jurisdiction::WALES => "Deddf Cyllid Llywodraeth Leol 1988 a Deddf Cyllid Llywodraeth Leol (Cymru) 2024 ar gyfer Cymru",
            Jurisdiction::SCOTLAND => "Deddf Cyllid Llywodraeth Leol 1988 / Deddfau Treth Gyngor ac Ardrethu ar gyfer yr Alban",
        };
        lines.push(format!("1. Fframwaith Statudol: Cyfrifwyd o dan {framework}."));
        lines.push(format!("2. Cyfnod Meddiannu / Atebolrwydd: {} i {} (Rhanwyd yn {} segment bilio).",
            start_d, end_d, segments.len()));

        for (idx, seg) in segments.iter().enumerate() {
            lines.push(format!("3.{} Blwyddyn Ariannol {}:", idx + 1, seg.financial_year));
            let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
                OccupancyType::OCCUPIED => "Wedi'i Feddiannu (Meddiannaeth Lawn)",
                OccupancyType::UNOCCUPIED => "Heb ei Feddiannu (Gwag)",
                OccupancyType::PART_OCCUPIED => "Rhan-Feddiant (Adran 44A)",
            };
            lines.push(format!("    - Statws Meddiannaeth: {}", occ_label));
            lines.push(format!("    - Gwerth Trethiannol (RV): £{}", seg.rateable_value));
            lines.push(format!("    - Lluosydd ({}): {}", seg.multiplier_used.multiplier_type, seg.multiplier_used.rate));
            lines.push(format!("    - Dyddiau Meddiannu: {} diwrnod ({} i {})", seg.segment_period.days, seg.segment_period.start_date, seg.segment_period.end_date));
            lines.push(format!("    - Atebolrwydd Crynswth: £{}", seg.gross_liability));

            if seg.transitional_adjustment.adjustment_type != "NONE" {
                let trans_welsh = if seg.transitional_adjustment.adjustment_type == "RELIEF" { "RHYDDHAD" } else { "PREMIWM" };
                lines.push(format!("    - Rhyddhad Trosiannol [{}]: -£{} [DCLL 1988 adr. 57A / Rheoliadau Symiau Ardrethi Annomestig]",
                    trans_welsh, seg.transitional_adjustment.amount));
            }

            if seg.itemized_reliefs.is_empty() {
                lines.push("    - Rhyddhadau a Gymhwyswyd: Dim".to_string());
            } else {
                for r in &seg.itemized_reliefs {
                    lines.push(format!("    - Rhyddhad [{}] {} ({}%): -£{} [{}]",
                        r.relief_code, r.relief_name, r.discount_percentage, r.amount_deducted, r.statutory_authority));
                }
            }

            let audit_notes = get_relief_non_award_explanations(req, seg, lang);
            if !audit_notes.is_empty() {
                lines.push("    - Archwiliad Asesu Rhyddhadau (Rhyddhadau heb eu Dyfarnu):".to_string());
                for note in audit_notes {
                    lines.push(format!("        * {}", note));
                }
            }

            if !seg.statutory_penalties.is_empty() {
                for p in &seg.statutory_penalties {
                    lines.push(format!("    - Cosb Statudol [{}] +£{} [{} - {}]",
                        p.penalty_type, p.amount, p.calculation_formula, p.statutory_authority));
                }
            }
            lines.push(format!("    - Swm Net y Segment i'w Dalu: £{}", seg.net_segment_payable));
        }

        if req.hereditament.scat_code == "249" {
            lines.push("4. Dilysu Ffiniau Statudol (SCAT 249 Llety Gwyliau):".to_string());
            let threshold = if req.hereditament.jurisdiction == crate::models::enums::Jurisdiction::SCOTLAND { 182 } else { 70 };
            if let Some(actual) = req.qualifying_relief_flags.holiday_let_actual_days_let {
                if actual < threshold {
                    lines.push(format!("    - Aseswyd o dan feini prawf statudol: wedi'i osod am {} diwrnod (llai na'r isafswm statudol o {} diwrnod).", actual, threshold));
                    lines.push("    - Statws: Wedi'i anghymhwyso o'r Rhestr Ardrethu Annomestig; trosglwyddwyd i Restr Brisio'r Dreth Gyngor (£0 o atebolrwydd NDR).".to_string());
                } else {
                    lines.push(format!("    - Aseswyd o dan feini prawf statudol: wedi'i osod am {} diwrnod (yn cyrraedd yr isafswm statudol o {} diwrnod).", actual, threshold));
                    lines.push("    - Statws: Llety gwyliau masnachol cymwys; cadwyd ar y Rhestr Ardrethu Annomestig.".to_string());
                }
            } else {
                lines.push("    - Aseswyd o dan Reoliadau Treth y Cyngor (Anheddau) (Yr Alban) 2026 / Rheol 140/70 diwrnod Lloegr.".to_string());
                lines.push("    - Statws: Llety gwyliau masnachol cymwys; cadwyd ar y Rhestr Ardrethu Annomestig.".to_string());
            }
        }

        lines.push("=============================================".to_string());
        lines.push(format!("CYFANSWM Y SWM NET I'W DALU: £{}", total_net));
    } else if lang.is_scots() {
        lines.push("STATUTARY CALCULATION NARRATIVE & AUDIT TRAIL".to_string());
        lines.push("=============================================".to_string());
        let framework = match req.hereditament.jurisdiction {
            crate::models::enums::Jurisdiction::ENGLAND => "Local Government Finance Act 1988 (as amendit bi Non-Domestic Rating Act 2023) for Ingland",
            crate::models::enums::Jurisdiction::WALES => "Local Government Finance Act 1988 an Local Government Finance (Wales) Act 2024 for Wales",
            crate::models::enums::Jurisdiction::SCOTLAND => "Non-Domestic Rates (Scotland) Act 2020 / Local Government Finance Acts for Scotland",
        };
        lines.push(format!("1. Statutary Framewark: Calculatit unner {framework}."));
        lines.push(format!("2. Leeabeelity Period: {} tae {} (Slicit intae {} billin segments).",
            start_d, end_d, segments.len()));

        for (idx, seg) in segments.iter().enumerate() {
            lines.push(format!("3.{} Financial Year {}:", idx + 1, seg.financial_year));
            let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
                crate::models::enums::OccupancyType::OCCUPIED => "Occupyit (Fou Occupation)",
                crate::models::enums::OccupancyType::UNOCCUPIED => "Unoccupyit (Tuim Proaperty)",
                crate::models::enums::OccupancyType::PART_OCCUPIED => "Pairt-Occupyit (Section 44A)",
            };
            lines.push(format!("    - Occupancy Status: {}", occ_label));
            lines.push(format!("    - Rateable Value: £{}", seg.rateable_value));
            lines.push(format!("    - Multiplier ({}): {}", seg.multiplier_used.multiplier_type, seg.multiplier_used.rate));
            lines.push(format!("    - Segment Duration: {} days ({} tae {})", seg.segment_period.days, seg.segment_period.start_date, seg.segment_period.end_date));
            lines.push(format!("    - Gross Leeabeelity: £{}", seg.gross_liability));

            if seg.transitional_adjustment.adjustment_type != "NONE" {
                lines.push(format!("    - Transitional Adjustment [{}]: -£{} [LGFA 1988 s.57A / NDR Chargeable Amounts Regs]",
                    seg.transitional_adjustment.adjustment_type, seg.transitional_adjustment.amount));
            }

            if seg.itemized_reliefs.is_empty() {
                lines.push("    - Applee'd Reliefs: Nane".to_string());
            } else {
                for r in &seg.itemized_reliefs {
                    lines.push(format!("    - Relief [{}] {} ({}%): -£{} [{}]",
                        r.relief_code, r.relief_name, r.discount_percentage, r.amount_deducted, r.statutory_authority));
                }
            }

            let audit_notes = get_relief_non_award_explanations(req, seg, lang);
            if !audit_notes.is_empty() {
                lines.push("    - Relief Assessment Audit (Unawarded Reliefs):".to_string());
                for note in audit_notes {
                    lines.push(format!("        * {}", note));
                }
            }

            if !seg.statutory_penalties.is_empty() {
                for p in &seg.statutory_penalties {
                    lines.push(format!("    - Statutary Penalty [{}] +£{} [{} - {}]",
                        p.penalty_type, p.amount, p.calculation_formula, p.statutory_authority));
                }
            }
            lines.push(format!("    - Net Segment Tae Pay: £{}", seg.net_segment_payable));
        }

        if req.hereditament.scat_code == "249" {
            lines.push("4. Statutary Borderline Verification (SCAT 249 Halyday Accommodation):".to_string());
            let threshold = if req.hereditament.jurisdiction == crate::models::enums::Jurisdiction::SCOTLAND { 182 } else { 70 };
            if let Some(actual) = req.qualifying_relief_flags.holiday_let_actual_days_let {
                if actual < threshold {
                    lines.push(format!("    - Assessed unner statutary criteria: actually let for {} days (ablow statutary minimum {} days).", actual, threshold));
                    lines.push("    - Status: Disqualified frae Non-Domestic Rating Leet; transferred tae Domestic Council Tax Valuation Leet (£0 NDR leeabeelity).".to_string());
                } else {
                    lines.push(format!("    - Assessed unner statutary criteria: actually let for {} days (meets minimum {} days).", actual, threshold));
                    lines.push("    - Status: Qualified commercial halyday let; retainit on Non-Domestic Rating Leet.".to_string());
                }
            } else {
                lines.push("    - Assessed unner Council Tax (Dwellings) (Scotland) Regs 2026 / 140/70 day rule.".to_string());
                lines.push("    - Status: Qualified commercial halyday let; retainit on Non-Domestic Rating Leet.".to_string());
            }
        }

        lines.push("=============================================".to_string());
        lines.push(format!("HALE NET AMOUNT TAE PAY: £{}", total_net));
    } else if lang.is_gaelic() {
        lines.push("AITHRIS REACHDAIL AIR ÀIREAMHACHADH ÌREAN & SLIGHE SGRÙDAIDH".to_string());
        lines.push("=============================================".to_string());
        let framework = match req.hereditament.jurisdiction {
            crate::models::enums::Jurisdiction::ENGLAND => "Achd Ionmhais Riaghaltas Ionadail 1988 (mar a chaidh atharrachadh le NDRA 2023) airson Shasainn",
            crate::models::enums::Jurisdiction::WALES => "Achd Ionmhais Riaghaltas Ionadail 1988 agus Achd 2024 airson na Cuimrigh",
            crate::models::enums::Jurisdiction::SCOTLAND => "Achd nan Ìrean Neo-Dachaigheil (Alba) 2020 / Achdan Ìrean na h-Alba",
        };
        lines.push(format!("1. Frèam Reachdail: Air a thomhas fo {framework}."));
        lines.push(format!("2. Ùine Buailteachd: {} gu {} (Air a roinn ann an {} earrannan bileachaidh).",
            start_d, end_d, segments.len()));

        for (idx, seg) in segments.iter().enumerate() {
            lines.push(format!("3.{} Bliadhna Ionmhais {}:", idx + 1, seg.financial_year));
            let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
                crate::models::enums::OccupancyType::OCCUPIED => "Gabhailte (Làn Gabhail)",
                crate::models::enums::OccupancyType::UNOCCUPIED => "Gun Ghabhail (Togalach Falamh)",
                crate::models::enums::OccupancyType::PART_OCCUPIED => "Pàirt-Ghabhailte (Earrann 44A)",
            };
            lines.push(format!("    - Inbhe Gabhaltais: {}", occ_label));
            lines.push(format!("    - Luach Measaidh (RV): £{}", seg.rateable_value));
            lines.push(format!("    - Iomadair ({}): {}", seg.multiplier_used.multiplier_type, seg.multiplier_used.rate));
            lines.push(format!("    - Fad an Earrann: {} làithean ({} gu {})", seg.segment_period.days, seg.segment_period.start_date, seg.segment_period.end_date));
            lines.push(format!("    - Buailteachd Làn: £{}", seg.gross_liability));

            if seg.transitional_adjustment.adjustment_type != "NONE" {
                lines.push(format!("    - Atharrachadh Eadar-amail [{}]: -£{} [LGFA 1988 e.57A / Riaghailtean Ìrean]",
                    seg.transitional_adjustment.adjustment_type, seg.transitional_adjustment.amount));
            }

            if seg.itemized_reliefs.is_empty() {
                lines.push("    - Faochaidhean an Sàs: Chan eil gin".to_string());
            } else {
                for r in &seg.itemized_reliefs {
                    lines.push(format!("    - Faochadh [{}] {} ({}%): -£{} [{}]",
                        r.relief_code, r.relief_name, r.discount_percentage, r.amount_deducted, r.statutory_authority));
                }
            }

            let audit_notes = get_relief_non_award_explanations(req, seg, lang);
            if !audit_notes.is_empty() {
                lines.push("    - Sgrùdadh Measaidh Faochaidhean (Faochaidhean gun Dleastanas):".to_string());
                for note in audit_notes {
                    lines.push(format!("        * {}", note));
                }
            }

            if !seg.statutory_penalties.is_empty() {
                for p in &seg.statutory_penalties {
                    lines.push(format!("    - Peanas Reachdail [{}] +£{} [{} - {}]",
                        p.penalty_type, p.amount, p.calculation_formula, p.statutory_authority));
                }
            }
            lines.push(format!("    - Glan an Earrann ri Phàigheadh: £{}", seg.net_segment_payable));
        }

        if req.hereditament.scat_code == "249" {
            lines.push("4. Dearbhadh Crìochan Reachdail (SCAT 249 Àite-fuirich Saor-làithean):".to_string());
            let threshold = if req.hereditament.jurisdiction == crate::models::enums::Jurisdiction::SCOTLAND { 182 } else { 70 };
            if let Some(actual) = req.qualifying_relief_flags.holiday_let_actual_days_let {
                if actual < threshold {
                    lines.push(format!("    - Aseswyd fo shlatan-tomhais reachdail: air mhàl airson {} làithean (nas lugha na an ìre as lugha de {} làithean).", actual, threshold));
                    lines.push("    - Inbhe: Dì-cheadaichte bhon Liosta Ìrean Neo-Dachaigheil; air a ghluasad gu Liosta Luachaidh Cìs Comhairle (£0 buailteachd NDR).".to_string());
                } else {
                    lines.push(format!("    - Aseswyd fo shlatan-tomhais reachdail: air mhàl airson {} làithean (a' coinneachadh ris an ìre as lugha de {} làithean).", actual, threshold));
                    lines.push("    - Inbhe: Màl saor-làithean malairteach barrantaichte air a chumail air an Liosta Ìrean.".to_string());
                }
            } else {
                lines.push("    - Air a mheasadh fo Riaghailtean Cìs Comhairle (Alba) 2026 / riaghailt 140/70 latha.".to_string());
                lines.push("    - Inbhe: Màl saor-làithean malairteach barrantaichte air a chumail air an Liosta Ìrean.".to_string());
            }
        }

        lines.push("=============================================".to_string());
        lines.push(format!("IOMLAN GLAN RI PHÀIGHEADH: £{}", total_net));
    } else {
        lines.push("STATUTORY CALCULATION NARRATIVE & AUDIT TRAIL".to_string());
        lines.push("=============================================".to_string());
        let framework = match req.hereditament.jurisdiction {
            crate::models::enums::Jurisdiction::ENGLAND => "Local Government Finance Act 1988 (as amended by Non-Domestic Rating Act 2023) for England",
            crate::models::enums::Jurisdiction::WALES => "Local Government Finance Act 1988 and Local Government Finance (Wales) Act 2024 for Wales",
            crate::models::enums::Jurisdiction::SCOTLAND => "Local Government Finance Act 1988 / Council Tax and Rating Acts for Scotland",
        };
        lines.push(format!("1. Statutory Framework: Calculated under {}.", framework));
        lines.push(format!("2. Liability Period: {} to {} (Sliced into {} billing segments).",
            start_d, end_d, segments.len()));

        for (idx, seg) in segments.iter().enumerate() {
            lines.push(format!("3.{} Financial Year {}:", idx + 1, seg.financial_year));
            let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
                crate::models::enums::OccupancyType::OCCUPIED => "Occupied (Full Occupation)",
                crate::models::enums::OccupancyType::UNOCCUPIED => "Unoccupied (Empty Property)",
                crate::models::enums::OccupancyType::PART_OCCUPIED => "Part-Occupied (Section 44A)",
            };
            lines.push(format!("    - Occupancy Status: {}", occ_label));
            lines.push(format!("    - Rateable Value: £{}", seg.rateable_value));
            lines.push(format!("    - Multiplier ({}): {}", seg.multiplier_used.multiplier_type, seg.multiplier_used.rate));
            lines.push(format!("    - Segment Duration: {} days ({} to {})", seg.segment_period.days, seg.segment_period.start_date, seg.segment_period.end_date));
            lines.push(format!("    - Gross Liability: £{}", seg.gross_liability));

            if seg.transitional_adjustment.adjustment_type != "NONE" {
                lines.push(format!("    - Transitional Adjustment [{}]: -£{} [LGFA 1988 s.57A / NDR Chargeable Amounts Regs]",
                    seg.transitional_adjustment.adjustment_type, seg.transitional_adjustment.amount));
            }

            if seg.itemized_reliefs.is_empty() {
                lines.push("    - Applied Reliefs: None".to_string());
            } else {
                for r in &seg.itemized_reliefs {
                    lines.push(format!("    - Relief [{}] {} ({}%): -£{} [{}]",
                        r.relief_code, r.relief_name, r.discount_percentage, r.amount_deducted, r.statutory_authority));
                }
            }

            let audit_notes = get_relief_non_award_explanations(req, seg, lang);
            if !audit_notes.is_empty() {
                lines.push("    - Relief Assessment Audit (Unawarded Reliefs):".to_string());
                for note in audit_notes {
                    lines.push(format!("        * {}", note));
                }
            }

            if !seg.statutory_penalties.is_empty() {
                for p in &seg.statutory_penalties {
                    lines.push(format!("    - Statutory Penalty [{}] +£{} [{} - {}]",
                        p.penalty_type, p.amount, p.calculation_formula, p.statutory_authority));
                }
            }
            lines.push(format!("    - Net Segment Payable: £{}", seg.net_segment_payable));
        }

        if req.hereditament.scat_code == "249" {
            lines.push("4. Statutory Borderline Verification (SCAT 249 Holiday Accommodation):".to_string());
            let threshold = if req.hereditament.jurisdiction == crate::models::enums::Jurisdiction::SCOTLAND { 182 } else { 70 };
            if let Some(actual) = req.qualifying_relief_flags.holiday_let_actual_days_let {
                if actual < threshold {
                    lines.push(format!("    - Assessed under statutory criteria: actually let for {} days (below statutory minimum {} days).", actual, threshold));
                    lines.push("    - Status: Disqualified from Non-Domestic Rating List; transferred to Domestic Council Tax Valuation List (£0 NDR liability).".to_string());
                } else {
                    lines.push(format!("    - Assessed under statutory criteria: actually let for {} days (meets minimum {} days).", actual, threshold));
                    lines.push("    - Status: Qualified commercial holiday let; retained on Non-Domestic Rating List.".to_string());
                }
            } else {
                lines.push("    - Assessed under The Council Tax (Dwellings) (Scotland) Regs 2026 / English 140/70 day rule.".to_string());
                lines.push("    - Status: Qualified commercial holiday let; retained on Non-Domestic Rating List.".to_string());
            }
        }

        lines.push("=============================================".to_string());
        lines.push(format!("TOTAL NET AMOUNT PAYABLE: £{}", total_net));
    }

    lines.join("\n")
}

fn resolve_segment_flags<'a>(
    req: &'a LiabilityCalculationRequest,
    seg: &AnnualBillingSegment,
) -> &'a QualifyingReliefFlags {
    if let Some(ref periods) = req.occupancy_periods {
        for p in periods {
            if seg.segment_period.start_date >= p.start_date && seg.segment_period.start_date <= p.end_date {
                if let Some(ref flags) = p.qualifying_relief_flags {
                    return flags;
                }
            }
        }
    }
    &req.qualifying_relief_flags
}

fn get_relief_non_award_explanations(
    req: &LiabilityCalculationRequest,
    seg: &AnnualBillingSegment,
    lang: Language,
) -> Vec<String> {
    let mut explanations = Vec::new();
    let flags = resolve_segment_flags(req, seg);
    let rv = seg.rateable_value;
    let occ = seg.occupancy_type.unwrap_or(req.occupancy_type);
    let is_unoccupied = occ == OccupancyType::UNOCCUPIED;
    let scat = &req.hereditament.scat_code;
    let jurisdiction = req.hereditament.jurisdiction;
    let zero_net = seg.net_segment_payable == Decimal::ZERO;

    // 1. Rural Settlement Relief
    if flags.is_rural_settlement && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "RURAL_SETTLEMENT") {
        let is_pub_or_petrol = scat == "209" || scat == "211";
        let rv_cap = if is_pub_or_petrol {
            Decimal::from(12500)
        } else {
            Decimal::from(8500)
        };
        let authority = if jurisdiction == Jurisdiction::SCOTLAND {
            "Local Government (Scotland) Act 1975 Sched 1A / LGFA 1997"
        } else {
            "LGFA 1988 s.43(6B) / LGFA 1997"
        };

        let note = if is_unoccupied {
            if lang.is_welsh() {
                "Rhyddhad Anheddiad Gwledig: Wedi'i anghymhwyso — eiddo'n wag (mae angen meddiannaeth weithredol ar gyfer rhyddhad gwledig gorfodol) [DCLL 1988 adr. 43(6B)].".to_string()
            } else if lang.is_scots() {
                "Rural Settlement Relief: Disqualifeet — proaperty is tuim/unoccupyit (mandatar rural settlement relief requires qualifyin occupation) [LGFA 1988 s.43(6B) / 1975 Act Sched 1A].".to_string()
            } else if lang.is_gaelic() {
                "Faochadh Tuineachaidh Dùthchail: Dì-cheadaichte — togalach gun ghabhail (feumaidh faochadh dùthchail èigneachail gabhaltas barrantaichte) [LGFA 1988 e.43(6B) / Achd 1975 Clàr 1A].".to_string()
            } else {
                "Rural Settlement Relief: Disqualified — property is unoccupied (mandatory rural settlement relief requires qualifying occupation) [LGFA 1988 s.43(6B) / Local Government (Scotland) Act 1975 Sched 1A].".to_string()
            }
        } else if rv > rv_cap {
            let cap_str = if is_pub_or_petrol { "£12,500" } else { "£8,500" };
            if lang.is_welsh() {
                let cat = if is_pub_or_petrol { "dafarnau / gorsafoedd petrol cymwys" } else { "hereditamentau masnachol/cyffredinol" };
                format!("Rhyddhad Anheddiad Gwledig: Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£{}) yn fwy na'r cap statudol o {} ar gyfer {} [{}].", rv, cap_str, cat, authority)
            } else if lang.is_scots() {
                let cat = if is_pub_or_petrol { "qualifyin public hooses / petrol stations" } else { "commercial/general hereditaments" };
                format!("Rural Settlement Relief: Disqualifeet — Rateable Value (£{}) exceeds statutary cap o {} for {} [{}].", rv, cap_str, cat, authority)
            } else if lang.is_gaelic() {
                let cat = if is_pub_or_petrol { "taighean-seinnse / stèiseanan peatrail" } else { "togalaichean malairteach/coitcheann" };
                format!("Faochadh Tuineachaidh Dùthchail: Dì-cheadaichte — tha Luach Measaidh (£{}) a' dol thairis air a' chrìoch reachdail de {} airson {} [{}].", rv, cap_str, cat, authority)
            } else {
                let cat = if is_pub_or_petrol { "qualifying public houses / petrol filling stations" } else { "commercial/general hereditaments" };
                format!("Rural Settlement Relief: Disqualified — Rateable Value (£{}) exceeds statutory cap of {} for {} [{}].", rv, cap_str, cat, authority)
            }
        } else if zero_net {
            if lang.is_welsh() {
                "Rhyddhad Anheddiad Gwledig: Heb ei gymhwyso — gostyngodd rhyddhadau blaenorol yr atebolrwydd net i £0.00.".to_string()
            } else if lang.is_scots() {
                "Rural Settlement Relief: No applee'd — prior mandatar reliefs reducit net leeabeelity tae £0.00.".to_string()
            } else if lang.is_gaelic() {
                "Faochadh Tuineachaidh Dùthchail: Gun a chur an sàs — lughdaich faochaidhean roimhe a' bhuailteachd ghlan gu £0.00.".to_string()
            } else {
                "Rural Settlement Relief: Not applied — prior mandatory reliefs reduced net payable liability to £0.00.".to_string()
            }
        } else {
            format!("Rural Settlement Relief: Disqualified — statutory qualification criteria not satisfied [{}].", authority)
        };
        explanations.push(note);
    }

    // 2. Small Business Rate Relief / SBBS
    if (flags.is_small_business_sole_property || flags.is_small_business_second_property_grace)
        && !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("SB"))
    {
        let note = if is_unoccupied {
            if lang.is_welsh() {
                "Rhyddhad Ardrethi Busnes Bach: Wedi'i anghymhwyso — eiddo gwag (mae SBRR yn gofyn am feddiant gweithredol cymwys) [DCLL 1988 adr. 43(4B) / DCLL(C) 2024].".to_string()
            } else if lang.is_scots() {
                "Small Business Bonus Scheme: Disqualifeet — tuim proaperty (SBBS requires active qualifyin occupation) [Local Government (Scotland) Act 1975 / SSI 2023/45].".to_string()
            } else if lang.is_gaelic() {
                "Sgeama Bònas Gnìomhachasan Beaga: Dì-cheadaichte — togalach falamh (feumaidh SBBS gabhaltas gnìomhach barrantaichte) [Achd Riaghaltas Ionadail (Alba) 1975 / SSI 2023/45].".to_string()
            } else {
                "Small Business Rate Relief: Disqualified — unoccupied property (SBRR/SBBS requires active qualifying occupation) [LGFA 1988 s.43(4B) / LGF(W)A 2024 / Local Government (Scotland) Act 1975].".to_string()
            }
        } else {
            match jurisdiction {
                Jurisdiction::ENGLAND => {
                    if rv > Decimal::from(15000) {
                        if lang.is_welsh() {
                            format!("Rhyddhad Ardrethi Busnes Bach: Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£{}) yn fwy na throthwy statudol o £15,000 [DCLL 1988 adr. 43(4B) / S.I. 2017/411].", rv)
                        } else if lang.is_scots() {
                            format!("Small Business Rate Relief: Disqualifeet — Rateable Value (£{}) exceeds £15,000 statutary taper ceilin [LGFA 1988 s.43(4B) / S.I. 2017/411].", rv)
                        } else if lang.is_gaelic() {
                            format!("Faochadh Ìrean Gnìomhachasan Beaga: Dì-cheadaichte — tha Luach Measaidh (£{}) a' dol thairis air mullach reachdail £15,000 [LGFA 1988 e.43(4B) / S.I. 2017/411].", rv)
                        } else {
                            format!("Small Business Rate Relief: Disqualified — Rateable Value (£{}) exceeds £15,000 statutory taper ceiling [LGFA 1988 s.43(4B) / S.I. 2017/411].", rv)
                        }
                    } else if zero_net {
                        "Small Business Rate Relief: Not applied — prior reliefs reduced net payable liability to £0.00.".to_string()
                    } else {
                        "Small Business Rate Relief: Disqualified — rateable value exceeds threshold for rating list period.".to_string()
                    }
                }
                Jurisdiction::WALES => {
                    if rv > Decimal::from(12000) {
                        if lang.is_welsh() {
                            format!("Rhyddhad Ardrethi Busnes Bach: Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£{}) yn fwy na throthwy uchaf statudol o £12,000 [DCLL(C) 2024 / Rheoliadau W.S.I.].", rv)
                        } else if lang.is_scots() {
                            format!("Welsh Small Business Rate Relief: Disqualifeet — Rateable Value (£{}) exceeds £12,000 statutary ceilin [LGF(W)A 2024].", rv)
                        } else if lang.is_gaelic() {
                            format!("Faochadh Gnìomhachasan Beaga na Cuimrigh: Dì-cheadaichte — tha Luach Measaidh (£{}) a' dol thairis air mullach reachdail £12,000 [DCLL(C) 2024].", rv)
                        } else {
                            format!("Small Business Rate Relief (Wales): Disqualified — Rateable Value (£{}) exceeds £12,000 statutory ceiling [LGF(W)A 2024 / W.S.I. Regs].", rv)
                        }
                    } else if zero_net {
                        "Small Business Rate Relief (Wales): Not applied — prior reliefs reduced net payable liability to £0.00.".to_string()
                    } else {
                        "Small Business Rate Relief (Wales): Disqualified — rateable value exceeds threshold for rating list period.".to_string()
                    }
                }
                Jurisdiction::SCOTLAND => {
                    if rv > Decimal::from(20000) {
                        if lang.is_welsh() {
                            format!("Sgeam Bonws Busnes Bach (SBBS): Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£{}) yn fwy na throthwy uchaf o £20,000 [Deddf 2020 yr Alban / SSI 2023/45].", rv)
                        } else if lang.is_scots() {
                            format!("Small Business Bonus Scheme (SBBS): Disqualifeet — Rateable Value (£{}) exceeds £20,000 statutary taper ceilin [Non-Domestic Rates (Scotland) Act 2020 / SSI 2023/45].", rv)
                        } else if lang.is_gaelic() {
                            format!("Sgeama Bònas Gnìomhachasan Beaga (SBBS): Dì-cheadaichte — tha Luach Measaidh (£{}) a' dol thairis air mullach reachdail £20,000 [Achd Ìrean Neo-Dachaigheil (Alba) 2020 / SSI 2023/45].", rv)
                        } else {
                            format!("Small Business Bonus Scheme (SBBS): Disqualified — Rateable Value (£{}) exceeds £20,000 statutory taper ceiling [Non-Domestic Rates (Scotland) Act 2020 / SSI 2023/45].", rv)
                        }
                    } else if zero_net {
                        "Small Business Bonus Scheme (SBBS): Not applied — prior reliefs reduced net payable liability to £0.00.".to_string()
                    } else {
                        "Small Business Bonus Scheme (SBBS): Disqualified — rateable value exceeds threshold for rating list period.".to_string()
                    }
                }
            }
        };
        explanations.push(note);
    }

    // 3. Retail, Hospitality & Leisure (RHL)
    if flags.is_rhl_qualifying && !seg.itemized_reliefs.iter().any(|r| r.relief_code.starts_with("RHL")) {
        let note = if is_unoccupied {
            if lang.is_welsh() {
                "Rhyddhad Manwerthu, Lletygarwch a Hamdden: Wedi'i anghymhwyso — eiddo gwag (mae angen meddiannaeth weithredol).".to_string()
            } else if lang.is_scots() {
                "Retail, Hospitality & Leisure Relief: Disqualifeet — tuim proaperty (RHL requires active occupation).".to_string()
            } else if lang.is_gaelic() {
                "Faochadh Reic, Aoigheachd agus Cur-seachad: Dì-cheadaichte — togalach falamh (feumaidh gabhaltas gnìomhach).".to_string()
            } else {
                "Retail, Hospitality & Leisure Relief: Disqualified — unoccupied property (RHL requires active qualifying occupation).".to_string()
            }
        } else {
            match jurisdiction {
                Jurisdiction::ENGLAND => {
                    let is_2026_plus = seg.financial_year.starts_with("2026") || seg.financial_year.starts_with("2027");
                    if is_2026_plus {
                        if lang.is_welsh() {
                            format!("Manwerthu, Lletygarwch a Hamdden: Disodlwyd gan luosydd stryd fawr statudol is ({} a gymhwyswyd uchod) — daeth y disgownt canran i ben 31 Mawrth 2026 [Deddf Ardrethi Annomestig 2023].", seg.multiplier_used.rate)
                        } else if lang.is_scots() {
                            format!("Retail, Hospitality & Leisure: Replacit bi statutary laicher high-street multiplier ({} applee'd abuin) — temporar percentage discount endit 31 Mairch 2026 [NDRA 2023].", seg.multiplier_used.rate)
                        } else if lang.is_gaelic() {
                            format!("Reic, Aoigheachd agus Cur-seachad: Air a chur an àite iomadair sràide àrd reachdail nas ìsle ({} air a chur an sàs gu h-àrd) — thàinig an lasachadh ceudad gu crìch 31 Màrt 2026 [NDRA 2023].", seg.multiplier_used.rate)
                        } else {
                            format!("Retail, Hospitality & Leisure: Replaced by permanent lower high-street multiplier ({} applied above) — temporary percentage discount ended 31 March 2026 [Non-Domestic Rating Act 2023].", seg.multiplier_used.rate)
                        }
                    } else if zero_net {
                        "Retail, Hospitality & Leisure Relief: Not applied — 100% relief already granted by prior reliefs.".to_string()
                    } else {
                        "Retail, Hospitality & Leisure Relief: Not eligible under scheme rules for this rating period.".to_string()
                    }
                }
                Jurisdiction::WALES => {
                    if scat == "211" {
                        if lang.is_welsh() {
                            "Rhyddhad Lletygarwch Bwyd a Diod (Cymru): Wedi'i anghymhwyso — mae gorsafoedd petrol (SCAT 211) wedi'u heithrio'n statudol o dan reolau'r cynllun [DCLL(C) 2024].".to_string()
                        } else if lang.is_scots() {
                            "Welsh Hospitality Relief: Disqualifeet — petrol fillin stations (SCAT 211) are statutary exclusions unner Welsh scheme rules [LGF(W)A 2024].".to_string()
                        } else if lang.is_gaelic() {
                            "Faochadh Aoigheachd na Cuimrigh: Dì-cheadaichte — tha stèiseanan peatrail (SCAT 211) air an dùnadh a-mach fo riaghailtean na Cuimrigh [DCLL(C) 2024].".to_string()
                        } else {
                            "Welsh Food & Drink Hospitality Relief: Disqualified — petrol filling stations (SCAT 211) are statutory exclusions under Welsh scheme rules [LGF(W)A 2024].".to_string()
                        }
                    } else if zero_net {
                        "Welsh Hospitality Relief: Not applied — prior reliefs reduced net payable liability to £0.00.".to_string()
                    } else {
                        "Welsh Hospitality Relief: Disqualified — hereditament already receives 100% small business rate relief or not qualifying.".to_string()
                    }
                }
                Jurisdiction::SCOTLAND => {
                    if !flags.is_island_property && rv > Decimal::from(100000) {
                        if lang.is_welsh() {
                            format!("Rhyddhad RHL yr Alban: Wedi'i anghymhwyso — mae'r Gwerth Trethiannol (£{}) yn fwy na chap tir mawr yr Alban o £100,000 [Deddf 2020 yr Alban].", rv)
                        } else if lang.is_scots() {
                            format!("Scottish RHL Relief: Disqualifeet — Rateable Value (£{}) exceeds Scots mainland £100,000 cap [NDR (Scotland) Act 2020].", rv)
                        } else if lang.is_gaelic() {
                            format!("Faochadh RHL na h-Alba: Dì-cheadaichte — tha Luach Measaidh (£{}) a' dol thairis air crìoch tìr-mòr na h-Alba de £100,000 [Achd Ìrean 2020].", rv)
                        } else {
                            format!("Scottish Retail, Hospitality & Leisure Relief: Disqualified — Rateable Value (£{}) exceeds Scottish mainland £100,000 cap [Non-Domestic Rates (Scotland) Act 2020].", rv)
                        }
                    } else if !flags.is_island_property && (seg.financial_year.starts_with("2024") || seg.financial_year.starts_with("2025") || seg.financial_year.starts_with("2026") || seg.financial_year.starts_with("2027")) {
                        if lang.is_welsh() {
                            "Rhyddhad RHL yr Alban: Wedi'i anghymhwyso — nid yw'r eiddo wedi'i leoli ar ynys yn yr Alban (cyfyngwyd rhyddhad 100% i ynysoedd) [Deddf Ynysoedd (Yr Alban) 2018].".to_string()
                        } else if lang.is_scots() {
                            "Scottish RHL Relief: Disqualifeet — proaperty isnae situatit on a Scots island (100% relief restrictit tae islands) [Islands (Scotland) Act 2018].".to_string()
                        } else if lang.is_gaelic() {
                            "Faochadh RHL na h-Alba: Dì-cheadaichte — chan eil an togalach air eilean Albannach (faochadh 100% cuibhrichte gu eileanan) [Achd nan Eilean (Alba) 2018].".to_string()
                        } else {
                            "Scottish Retail, Hospitality & Leisure Relief: Disqualified — property is not situated on a Scottish island (100% relief restricted to islands) [Islands (Scotland) Act 2018].".to_string()
                        }
                    } else if zero_net {
                        "Scottish RHL Relief: Not applied — prior reliefs reduced net liability to £0.00.".to_string()
                    } else {
                        "Scottish RHL Relief: Not eligible under scheme rules for this financial year.".to_string()
                    }
                }
            }
        };
        explanations.push(note);
    }

    // 4. Mandatory Charitable Relief (80%)
    let charity_claimed = flags.is_charitable_purposes || req.party_type == PartyType::CHARITY || req.party_type == PartyType::CASC;
    if charity_claimed && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "CHARITY_MANDATORY") {
        let note = if is_unoccupied {
            if lang.is_welsh() {
                "Rhyddhad Elusennol Gorfodol: Wedi'i anghymhwyso — eiddo gwag (mae rhyddhad elusennol 80% yn gofyn am feddiannaeth weithredol) [DCLL 1988 adr. 43(5)].".to_string()
            } else if lang.is_scots() {
                "Mandatar Chairitable Relief: Disqualifeet — tuim proaperty (mandatar 80% relief requires qualifyin occupation) [1962 Act s.4(2)].".to_string()
            } else if lang.is_gaelic() {
                "Faochadh Carthannais Èigneachail: Dì-cheadaichte — togalach falamh (feumaidh faochadh carthannais 80% gabhaltas barrantaichte) [Achd 1962 e.4(2)].".to_string()
            } else {
                "Mandatory Charitable Relief: Disqualified — unoccupied property (mandatory 80% charitable relief requires qualifying occupation) [LGFA 1988 s.43(5) / 1962 Act s.4(2)].".to_string()
            }
        } else if zero_net {
            "Mandatory Charitable Relief: Not applied — prior reliefs reduced net payable liability to £0.00.".to_string()
        } else {
            "Mandatory Charitable Relief: Not applied under statutory rules.".to_string()
        };
        explanations.push(note);
    }

    // 5. Charity Discretionary Top-Up (20%)
    if flags.is_charity_discretionary_top_up && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "CHARITY_DISCRETIONARY_TOPUP") {
        let note = if !charity_claimed {
            if lang.is_welsh() {
                "Top-Up Elusennol Dewisol: Wedi'i anghymhwyso — mae angen bod yn gymwys ar gyfer Rhyddhad Elusennol Gorfodol o 80% [DCLL 1988 adr. 47(2)(a)].".to_string()
            } else if lang.is_scots() {
                "Discretionar Chairitable Top-Up: Disqualifeet — requires qualification for 80% Mandatar Chairitable Relief [LGFA 1988 s.47(2)(a)].".to_string()
            } else if lang.is_gaelic() {
                "Top-Up Carthannais Fo Ùghdarras: Dì-cheadaichte — feumaidh teisteanas airson Faochadh Carthannais Èigneachail 80% [LGFA 1988 e.47(2)(a)].".to_string()
            } else {
                "Discretionary Charitable Top-Up: Disqualified — requires qualification for 80% Mandatory Charitable Relief [LGFA 1988 s.47(2)(a) / 1962 Act s.4(5)].".to_string()
            }
        } else if zero_net {
            "Discretionary Charitable Top-Up: Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Discretionary Charitable Top-Up: Disqualified — mandatory relief not active.".to_string()
        };
        explanations.push(note);
    }

    // 6. Improvement Relief (G-Factor)
    if flags.qualifying_improvement_rv_increment > Decimal::ZERO && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "IMPROVEMENT_G_FACTOR") {
        let note = if jurisdiction != Jurisdiction::ENGLAND {
            "Improvement Relief (G-Factor): Disqualified — Improvement Relief applies only in England [Non-Domestic Rating Act 2023 s.1 / LGFA 1988 Sched 4ZA].".to_string()
        } else if seg.financial_year.starts_with("19") || seg.financial_year.starts_with("200") || seg.financial_year.starts_with("201") || seg.financial_year.starts_with("2020") || seg.financial_year.starts_with("2021") || seg.financial_year.starts_with("2022") || seg.financial_year.starts_with("2023") {
            "Improvement Relief (G-Factor): Disqualified — Statutory relief takes effect from 1 April 2024 [Non-Domestic Rating Act 2023].".to_string()
        } else if is_unoccupied {
            "Improvement Relief (G-Factor): Disqualified — requires qualifying continuing occupation.".to_string()
        } else if zero_net {
            "Improvement Relief (G-Factor): Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Improvement Relief (G-Factor): Not qualifying under statutory rules.".to_string()
        };
        explanations.push(note);
    }

    // 7. Heat Network Exemption
    if flags.is_heat_network && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "HEAT_NETWORK") {
        let note = if jurisdiction != Jurisdiction::ENGLAND {
            "Heat Network Exemption: Disqualified — 100% exemption applies only in England [Non-Domestic Rating Act 2023 s.2].".to_string()
        } else if is_unoccupied {
            "Heat Network Exemption: Disqualified — requires active qualifying occupation.".to_string()
        } else if zero_net {
            "Heat Network Exemption: Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Heat Network Exemption: Not qualifying under statutory rules.".to_string()
        };
        explanations.push(note);
    }

    // 8. Public Lavatory Relief
    if flags.is_public_lavatory && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "PUBLIC_LAVATORY") {
        let note = if jurisdiction == Jurisdiction::SCOTLAND {
            "Public Lavatory Relief: Disqualified — 100% relief applies under Rating (Public Lavatories) Act 2021 (England and Wales only).".to_string()
        } else if is_unoccupied {
            "Public Lavatory Relief: Disqualified — requires qualifying occupation.".to_string()
        } else if zero_net {
            "Public Lavatory Relief: Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Public Lavatory Relief: Not qualifying under statutory rules.".to_string()
        };
        explanations.push(note);
    }

    // 9. Scottish Renewable Energy Generation Relief
    if flags.is_renewable_energy_generation && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "RENEWABLE_ENERGY_RELIEF") {
        let note = if jurisdiction != Jurisdiction::SCOTLAND {
            "Renewable Energy Generation Relief: Disqualified — applies only in Scotland [SSI 2010/336].".to_string()
        } else if rv > Decimal::from(1000000) {
            format!("Renewable Energy Generation Relief: Disqualified — Rateable Value (£{}) exceeds Scottish £1,000,000 statutory limit [SSI 2010/336].", rv)
        } else if is_unoccupied {
            "Renewable Energy Generation Relief: Disqualified — requires qualifying occupation.".to_string()
        } else if zero_net {
            "Renewable Energy Generation Relief: Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Renewable Energy Generation Relief: Disqualified under scheme rules.".to_string()
        };
        explanations.push(note);
    }

    // 10. Telecommunications Infrastructure Relief
    if flags.is_telecommunications_infrastructure && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "TELECOM_FIBRE_RELIEF") {
        let note = if is_unoccupied {
            "Telecommunications Infrastructure Relief: Disqualified — requires active qualifying occupation.".to_string()
        } else if zero_net {
            "Telecommunications Infrastructure Relief: Not applied — net liability already reduced to £0.00.".to_string()
        } else {
            "Telecommunications Infrastructure Relief: Not qualifying under statutory rules.".to_string()
        };
        explanations.push(note);
    }

    // 11. Scottish SBBS Transitional Relief (SBBS TR)
    if flags.is_sbbs_tr_qualifying && !seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SBBS_TR")) {
        let note = if jurisdiction != Jurisdiction::SCOTLAND {
            "SBBS Transitional Relief: Disqualified — applies only in Scotland [SSI 2023/45].".to_string()
        } else {
            "SBBS Transitional Relief: Not applicable — bill increase did not exceed statutory cap over previous year.".to_string()
        };
        explanations.push(note);
    }

    // 12. Supporting Small Business Relief (SSBR) in England
    if flags.is_ssbr_qualifying && !seg.itemized_reliefs.iter().any(|r| r.relief_code.contains("SSBR")) {
        let note = if jurisdiction != Jurisdiction::ENGLAND {
            "Supporting Small Business Relief: Disqualified — applies only in England [LGFA 1988 s.47].".to_string()
        } else {
            "Supporting Small Business Relief: Not applicable — bill increase did not exceed statutory annual cap (£600/£800) over previous financial year [LGFA 1988 s.47].".to_string()
        };
        explanations.push(note);
    }

    // 13. Empty Property Exemption (when unoccupied)
    if is_unoccupied && !seg.itemized_reliefs.iter().any(|r| r.relief_code == "EMPTY_PROPERTY_EXEMPTION" || r.relief_code == "EMPTY_LOW_VALUE") {
        let note = if lang.is_welsh() {
            "Cyfnod Di-dreth Eiddo Gwag: Wedi dod i ben — mae'r cyfnod statudol (3 mis masnachol / 6 mis diwydiannol) wedi dod i ben [DCLL 1988 adr. 45].".to_string()
        } else if lang.is_scots() {
            "Tuim Proaperty Rate-Free Period: Expired — statutary rate-free period (3 month commercial / 6 month industrial) is up [1975 Act s.7A / SSI 2018/77].".to_string()
        } else if lang.is_gaelic() {
            "Ùine Saor bho Ìrean Togalach Falamh: Dh'fhalbh an ùine — tha an ùine reachdail (3 mìosan malairteach / 6 mìosan gnìomhachais) seachad [Achd 1975 e.7A].".to_string()
        } else {
            "Empty Property Rate-Free Period: Expired — initial statutory rate-free period (3 months commercial / 6 months industrial) has elapsed [LGFA 1988 s.45 / SI 2008/386 / 1975 Act s.7A].".to_string()
        };
        explanations.push(note);
    }

    explanations
}

