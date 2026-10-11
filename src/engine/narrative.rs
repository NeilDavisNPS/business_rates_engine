use rust_decimal::Decimal;
use crate::models::calculation::{LiabilityCalculationRequest, AnnualBillingSegment};
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
            crate::models::enums::Jurisdiction::ENGLAND => "Deddf Cyllid Llywodraeth Leol 1988 (fel y'i diwygiwyd gan Ddeddf Ardrethi Annomestig 2023) ar gyfer Lloegr",
            crate::models::enums::Jurisdiction::WALES => "Deddf Cyllid Llywodraeth Leol 1988 a Deddf Cyllid Llywodraeth Leol (Cymru) 2024 ar gyfer Cymru",
            crate::models::enums::Jurisdiction::SCOTLAND => "Deddf Cyllid Llywodraeth Leol 1988 / Deddfau Treth Gyngor ac Ardrethu ar gyfer yr Alban",
        };
        lines.push(format!("1. Fframwaith Statudol: Cyfrifwyd o dan {framework}."));
        lines.push(format!("2. Cyfnod Meddiannu / Atebolrwydd: {} i {} (Rhanwyd yn {} segment bilio).",
            start_d, end_d, segments.len()));

        for (idx, seg) in segments.iter().enumerate() {
            lines.push(format!("3.{} Blwyddyn Ariannol {}:", idx + 1, seg.financial_year));
            let occ_label = match seg.occupancy_type.unwrap_or(req.occupancy_type) {
                crate::models::enums::OccupancyType::OCCUPIED => "Wedi'i Feddiannu (Meddiannaeth Lawn)",
                crate::models::enums::OccupancyType::UNOCCUPIED => "Heb ei Feddiannu (Gwag)",
                crate::models::enums::OccupancyType::PART_OCCUPIED => "Rhan-Feddiant (Adran 44A)",
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
            lines.push("    - Aseswyd o dan Reoliadau Treth y Cyngor (Anheddau) (Yr Alban) 2026 / Rheol 140/70 diwrnod Lloegr.".to_string());
            lines.push("    - Statws: Llety gwyliau masnachol cymwys; cadwyd ar y Rhestr Ardrethu Annomestig.".to_string());
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
            lines.push("    - Assessed unner Council Tax (Dwellings) (Scotland) Regs 2026 / 140/70 day rule.".to_string());
            lines.push("    - Status: Qualified commercial halyday let; retainit on Non-Domestic Rating Leet.".to_string());
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
            lines.push("    - Air a mheasadh fo Riaghailtean Cìs Comhairle (Alba) 2026 / riaghailt 140/70 latha.".to_string());
            lines.push("    - Inbhe: Màl saor-làithean malairteach barrantaichte air a chumail air an Liosta Ìrean.".to_string());
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
            lines.push("    - Assessed under The Council Tax (Dwellings) (Scotland) Regs 2026 / English 140/70 day rule.".to_string());
            lines.push("    - Status: Qualified commercial holiday let; retained on Non-Domestic Rating List.".to_string());
        }

        lines.push("=============================================".to_string());
        lines.push(format!("TOTAL NET AMOUNT PAYABLE: £{}", total_net));
    }

    lines.join("\n")
}
