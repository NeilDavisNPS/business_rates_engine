use serde::{Deserialize, Serialize};

/// Supported languages for statutory business rates calculations and UI rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    En,
    Cy,
    Sco,
    Gd,
}

impl Language {
    /// Parses language code from string ("en", "cy", "sco", "gd", "welsh", "scots", "gaelic")
    pub fn from_code(s: &str) -> Self {
        let trimmed = s.trim().to_lowercase();
        if trimmed.starts_with("cy") || trimmed == "welsh" || trimmed == "cymraeg" {
            Language::Cy
        } else if trimmed.starts_with("sco") || trimmed == "scots" {
            Language::Sco
        } else if trimmed.starts_with("gd")
            || trimmed == "gaelic"
            || trimmed == "gla"
            || trimmed == "gàidhlig"
            || trimmed == "gaidhlig"
            || trimmed == "scottish gaelic"
        {
            Language::Gd
        } else {
            Language::En
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Cy => "cy",
            Language::Sco => "sco",
            Language::Gd => "gd",
        }
    }

    pub fn is_welsh(&self) -> bool {
        matches!(self, Language::Cy)
    }

    pub fn is_scots(&self) -> bool {
        matches!(self, Language::Sco)
    }

    pub fn is_gaelic(&self) -> bool {
        matches!(self, Language::Gd)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Cy => "Cymraeg",
            Language::Sco => "Scots",
            Language::Gd => "Gàidhlig",
        }
    }
}

/// Translates statutory reference codes, categories, and descriptions to Welsh, Scots, Scottish Gaelic, or English
pub fn localize_reference_code(
    category: &str,
    code: &str,
    en_label: &str,
    en_desc: &str,
    lang: Language,
) -> (String, String) {
    match lang {
        Language::En => (en_label.to_string(), en_desc.to_string()),
        Language::Cy => localize_reference_code_cy(category, code, en_label, en_desc),
        Language::Sco => localize_reference_code_sco(category, code, en_label, en_desc),
        Language::Gd => localize_reference_code_gd(category, code, en_label, en_desc),
    }
}

fn localize_reference_code_cy(
    category: &str,
    code: &str,
    en_label: &str,
    en_desc: &str,
) -> (String, String) {
    match (category, code) {
        ("JURISDICTION", "ENGLAND") => (
            "Lloegr".to_string(),
            "Ardrethu annomestig statudol o dan Ddeddf Cyllid Llywodraeth Leol 1988 a Deddf Ardrethi Annomestig 2023. Mae'r lluosydd safonol yn berthnasol ar gyfer RV > £51,000.".to_string(),
        ),
        ("JURISDICTION", "WALES") => (
            "Cymru".to_string(),
            "Ardrethu datganoledig o dan Ddeddf Cyllid Llywodraeth Leol (Cymru) 2024 ac Offerynnau Statudol Cymru (OSC).".to_string(),
        ),
        ("JURISDICTION", "SCOTLAND") => (
            "Yr Alban".to_string(),
            "Ardrethu datganoledig o dan Ddeddf Ardrethi Annomestig (Yr Alban) 2020. Strwythur lluosydd tair haen a Chynllun Bonws Busnesau Bach.".to_string(),
        ),

        ("PARTY_TYPE", "CORPORATE") => (
            "Endid Corfforaethol / Cwmni".to_string(),
            "Cwmni cyfyngedig (Cyf/Ltd), Ccc/PLC, PAC/LLP, neu endid masnachol corfforedig arall sy'n atebol am ardrethi annomestig.".to_string(),
        ),
        ("PARTY_TYPE", "INDIVIDUAL") => (
            "Unigolyn / Perchennog Unigol".to_string(),
            "Person naturiol, unig fasnachwr, neu bartneriaeth fasnachol anghorfforedig.".to_string(),
        ),
        ("PARTY_TYPE", "CHARITY") => (
            "Elusen Gofrestredig".to_string(),
            "Elusen gofrestredig neu ymddiriedolwyr sy'n dal eiddo at ddibenion elusennol, sy'n gymwys am ryddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5).".to_string(),
        ),
        ("PARTY_TYPE", "CASC") => (
            "Clwb Chwaraeon Amatur Cymunedol (CASC)".to_string(),
            "CASC wedi'i gofrestru gyda CThEM sy'n gymwys am ryddhad ardrethi annomestig gorfodol o 80% o dan DCLL 1988 adr. 43(6).".to_string(),
        ),
        ("PARTY_TYPE", "LOCAL_AUTHORITY") => (
            "Awdurdod Lleol / Corff Cyhoeddus".to_string(),
            "Awdurdod bilio, cyngor bwrdeistrefol, neu feddiannydd corff cyhoeddus statudol arall.".to_string(),
        ),

        ("OCCUPANCY_TYPE", "OCCUPIED") => (
            "Mangre a Feddienir".to_string(),
            "Mae'r eiddo'n cael ei feddiannu'n weithredol ar gyfer gweithrediadau busnes/masnachol (atebolrwydd adr. 43).".to_string(),
        ),
        ("OCCUPANCY_TYPE", "UNOCCUPIED") => (
            "Gwag / Heb ei Feddiannu".to_string(),
            "Mae'r eiddo'n wag (mae atebolrwydd adr. 45 yn gymwys ar ôl eithriad statudol o 3 mis neu 6 mis).".to_string(),
        ),
        ("OCCUPANCY_TYPE", "PART_OCCUPIED") => (
            "Wedi'i Rannol Feddiannu".to_string(),
            "Mae'r eiddo'n wag dros dro yn rhannol, yn amodol ar dystysgrifau dosrannu adran 44A.".to_string(),
        ),

        ("SCAT_CODE", "003") => (
            "003 - Maes Awyr / Seilwaith Hedfan Mawr".to_string(),
            "Maes awyr / cyfadeilad gwerth uchel sy'n gofyn am luosydd cenedlaethol safonol. Wedi'i eithrio'n benodol o ryddhad busnesau bach.".to_string(),
        ),
        ("SCAT_CODE", "091") => (
            "091 - Swyddfeydd Cyffredinol / Mangre Fasnachol".to_string(),
            "Llety swyddfa fasnachol safonol. Hawl i 3 mis o ryddhad eiddo gwag 100%.".to_string(),
        ),
        ("SCAT_CODE", "096") => (
            "096 - Eiddo Diwydiannol / Warws / Ffatri".to_string(),
            "Eiddo diwydiannol sy'n gymwys am eithriad eiddo gwag 100% am 6 mis o dan O.S. 2008/386.".to_string(),
        ),
        ("SCAT_CODE", "209") => (
            "209 - Tafarn / Bar / Safle Trwyddedig".to_string(),
            "Mangre drwyddedig sy'n gymwys ar gyfer cynlluniau rhyddhad Manwerthu, Lletygarwch a Hamdden (RHL).".to_string(),
        ),
        ("SCAT_CODE", "211") => (
            "211 - Gorsaf Danwydd a Chwrt Blaen".to_string(),
            "Gwerthu tanwydd modur a siop cwrt blaen. Wedi'i eithrio'n benodol o ddisgownt manwerthu yng Nghymru o dan reolau 2026/27.".to_string(),
        ),
        ("SCAT_CODE", "225") => (
            "225 - Siop Fanwerthu / Uned Stryd Fawr".to_string(),
            "Siop fanwerthu safonol sy'n gymwys ar gyfer cynlluniau disgownt SBRR a RHL.".to_string(),
        ),
        ("SCAT_CODE", "249") => (
            "249 - Llety Gwyliau Hunan-ddarpar".to_string(),
            "Llety gwyliau yn amodol ar feini prawf gosod statudol lleiaf (o leiaf 70 diwrnod gosod gwirioneddol / 140 diwrnod ar gael).".to_string(),
        ),

        ("RELIEF_CODE", "SBRR") => (
            "Rhyddhad Ardrethi Busnesau Bach (Lloegr)".to_string(),
            "Disgownt o 100% hyd at £12,000 RV, gyda thapr llinol hyd at £15,000 RV ar gyfer meddianwyr un eiddo (Lloegr).".to_string(),
        ),
        ("RELIEF_CODE", "WALES_SBRR") => (
            "Rhyddhad Ardrethi Busnesau Bach (Cymru)".to_string(),
            "Rhyddhad o 100% hyd at £6,000 RV, gyda thapr llinol hyd at £12,000 RV o dan Ddeddf Cyllid Llywodraeth Leol (Cymru) 2024.".to_string(),
        ),
        ("RELIEF_CODE", "SBBS") => (
            "Cynllun Bonws Busnesau Bach (Yr Alban)".to_string(),
            "Rhyddhad ardrethu'r Alban ar gyfer busnesau bach hyd at £12,000 RV (2023-2026) neu £11,500 RV (2026+).".to_string(),
        ),
        ("RELIEF_CODE", "RHL") => (
            "Rhyddhad Manwerthu, Lletygarwch a Hamdden".to_string(),
            "Disgownt sector statudol (40% yng Nghymru 2026/27, yn amodol ar gap arian parod o £110,000).".to_string(),
        ),
        ("RELIEF_CODE", "CHARITY") => (
            "Rhyddhad Elusennol Gorfodol".to_string(),
            "Rhyddhad gorfodol o 80% o dan DCLL 1988 adr. 43(5) lle defnyddir yr eiddo'n gyfan gwbl neu'n bennaf at ddibenion elusennol.".to_string(),
        ),
        ("RELIEF_CODE", "IMPROV") => (
            "Rhyddhad Gwelliannau (Ffactor-G)".to_string(),
            "Rhyddhad o 100% am 12 mis ar gynnydd mewn gwerth trethiannol o ganlyniad i welliannau corfforol i eiddo o dan NDRA 2023.".to_string(),
        ),
        ("RELIEF_CODE", "EMPTY") => (
            "Eithriad Eiddo Gwag".to_string(),
            "Rhyddhad o 100% am y 3 mis cyntaf (masnachol) neu 6 mis (diwydiannol) o gyfnodau heb eu meddiannu o dan DCLL 1988 adr. 45.".to_string(),
        ),
        ("RELIEF_CODE", "RURAL") => (
            "Rhyddhad Aneddiadau Gwledig".to_string(),
            "Rhyddhad gorfodol o 100% ar gyfer unig siopau cyffredinol, siopau bwyd, neu swyddfeydd post mewn aneddiadau gwledig dynodedig.".to_string(),
        ),
        ("RELIEF_CODE", "LAVATORY") => (
            "Rhyddhad Toiledau Cyhoeddus".to_string(),
            "Eithriad ardrethu statudol o 100% o dan Ddeddf Ardrethu (Coronafeirws a Thoiledau Cyhoeddus) 2021.".to_string(),
        ),
        ("RELIEF_CODE", "HEAT_NET") => (
            "Rhyddhad Rhwydwaith Gwres".to_string(),
            "Rhyddhad o 100% ar gyfer eiddo dosbarthu gwres ardal carbon isel o dan NDRA 2023.".to_string(),
        ),
        ("RELIEF_CODE", "WELSH_AVOIDANCE") => (
            "Cosb Statudol Gwrth-osgoi (Cymru)".to_string(),
            "Cosb statudol o dan Offeryn Statudol Cymru 2026/43 (£500 sefydlog + 3% gwerth trethiannol) am osgoi artiffisial.".to_string(),
        ),

        _ => (en_label.to_string(), en_desc.to_string()),
    }
}

fn localize_reference_code_sco(
    category: &str,
    code: &str,
    en_label: &str,
    en_desc: &str,
) -> (String, String) {
    match (category, code) {
        ("JURISDICTION", "ENGLAND") => (
            "Ingland".to_string(),
            "Statutary non-domestic ratin unner Local Government Finance Act 1988 and Non-Domestic Rating Act 2023.".to_string(),
        ),
        ("JURISDICTION", "WALES") => (
            "Wales".to_string(),
            "Devolved non-domestic ratin unner Local Government Finance (Wales) Act 2024 and WSI statutory instruments.".to_string(),
        ),
        ("JURISDICTION", "SCOTLAND") => (
            "Scotland".to_string(),
            "Devolved non-domestic ratin unner Non-Domestic Rates (Scotland) Act 2020. Three-tier multiplier seestem an Small Business Bonus Scheme.".to_string(),
        ),
        ("PARTY_TYPE", "CORPORATE") => (
            "Corporate Body / Ltd".to_string(),
            "Leevemit company (Ltd/PLC/LLP) liable for statutary non-domestic rates.".to_string(),
        ),
        ("PARTY_TYPE", "INDIVIDUAL") => (
            "Individual / Sole Trader".to_string(),
            "Naitural body, single trader, or unincorporatit partnership.".to_string(),
        ),
        ("PARTY_TYPE", "CHARITY") => (
            "Registratit Chairity".to_string(),
            "Chairity registratit wi OSCR/CC entitlit tae 80% mandatar relief unner statute.".to_string(),
        ),
        ("OCCUPANCY_TYPE", "OCCUPIED") => (
            "Occupyit (Fou)".to_string(),
            "Hereditament is actively occupyit for commercial purposes.".to_string(),
        ),
        ("OCCUPANCY_TYPE", "UNOCCUPIED") => (
            "Tuim / Unoccupyit".to_string(),
            "Hereditament is tuim (empty rates relief an statutory liability applies).".to_string(),
        ),
        ("OCCUPANCY_TYPE", "PART_OCCUPIED") => (
            "Pairt-Occupyit (s.44A)".to_string(),
            "Hereditament is temporarily pairt-tuim, subject tae statutory apportionment.".to_string(),
        ),
        ("RELIEF_CODE", "SBBS") => (
            "Small Business Bonus Scheme (Scotland)".to_string(),
            "Scots rates relief for wee businesses up tae £12,000/£11,500 RV.".to_string(),
        ),
        ("RELIEF_CODE", "SBRR") => (
            "Small Business Rate Relief (Ingland)".to_string(),
            "100% discoont up tae £12k RV; linear taper tae £15k RV for single-proaperty occupiers.".to_string(),
        ),
        ("RELIEF_CODE", "CHARITY") => (
            "Mandatar Chairitable Relief".to_string(),
            "80% mandatar relief unner LGFA 1988 / LGA(S) 1962 s.4 for chairitable uiss.".to_string(),
        ),
        _ => (en_label.to_string(), en_desc.to_string()),
    }
}

fn localize_reference_code_gd(
    category: &str,
    code: &str,
    en_label: &str,
    en_desc: &str,
) -> (String, String) {
    match (category, code) {
        ("JURISDICTION", "ENGLAND") => (
            "Sasainn".to_string(),
            "Ìrean neo-dachaigheil reachdail fo Achd Ionmhais Riaghaltas Ionadail 1988 agus Achd nan Ìrean Neo-Dachaigheil 2023.".to_string(),
        ),
        ("JURISDICTION", "WALES") => (
            "A' Chuimrigh".to_string(),
            "Ìrean tiomnaichte fo Achd Ionmhais Riaghaltas Ionadail (A' Chuimrigh) 2024 agus Innealan Reachdail na Cuimrigh.".to_string(),
        ),
        ("JURISDICTION", "SCOTLAND") => (
            "Alba".to_string(),
            "Ìrean tiomnaichte fo Achd nan Ìrean Neo-Dachaigheil (Alba) 2020. Siostam iomadair trì-ìre agus Sgeama Bònas Ghnìomhachasan Beaga.".to_string(),
        ),
        ("PARTY_TYPE", "CORPORATE") => (
            "Buidheann Corporra / Companaidh".to_string(),
            "Companaidh earranta (Ltd/PLC/LLP) buailteach airson ìrean neo-dachaigheil reachdail.".to_string(),
        ),
        ("PARTY_TYPE", "INDIVIDUAL") => (
            "Neach Fa leth / Neach-malairt Aonar".to_string(),
            "Neach nàdarra, neach-malairt aonar, no com-pàirteachas neo-chorporra.".to_string(),
        ),
        ("PARTY_TYPE", "CHARITY") => (
            "Carthannas Clàraichte".to_string(),
            "Carthannas clàraichte le OSCR/CC le còir air faochadh èigneachail 80% fo reachd.".to_string(),
        ),
        ("OCCUPANCY_TYPE", "OCCUPIED") => (
            "Gabhailte (Làn)".to_string(),
            "Tha an togalach air a ghabhail gu gnìomhach airson gnìomhachas.".to_string(),
        ),
        ("OCCUPANCY_TYPE", "UNOCCUPIED") => (
            "Falamh / Gun Ghabhail".to_string(),
            "Tha an togalach falamh (buailteachd reachdail an sàs às deidh saoradh).".to_string(),
        ),
        ("OCCUPANCY_TYPE", "PART_OCCUPIED") => (
            "Pàirt-ghabhailte (e.44A)".to_string(),
            "Tha an togalach falamh gu sealach ann am pàirt, fo theisteanasan apportionment.".to_string(),
        ),
        ("RELIEF_CODE", "SBBS") => (
            "Sgeama Bònas Ghnìomhachasan Beaga (Alba)".to_string(),
            "Faochadh ìrean Albannach airson gnìomhachasan beaga suas gu £12,000/£11,500 RV.".to_string(),
        ),
        ("RELIEF_CODE", "SBRR") => (
            "Faochadh Ìrean Gnìomhachais Bheaga (Sasainn)".to_string(),
            "Lasachadh 100% suas gu £12,000 RV le teip loidhneach gu £15,000 RV.".to_string(),
        ),
        ("RELIEF_CODE", "CHARITY") => (
            "Faochadh Carthannais Èigneachail".to_string(),
            "Faochadh èigneachail 80% fo LGFA 1988 e.43(5) no Achd 1962 e.4 airson adhbharan carthannach.".to_string(),
        ),
        _ => (en_label.to_string(), en_desc.to_string()),
    }
}

/// Translates itemized statutory relief names to Welsh, Scots, Scottish Gaelic, or English
pub fn localize_relief_name(name: &str, lang: Language) -> String {
    match lang {
        Language::En => name.to_string(),
        Language::Cy => match name {
            "Mandatory Charitable Rate Relief" => "Rhyddhad Elusennol Gorfodol".to_string(),
            "Small Business Rate Relief" => "Rhyddhad Ardrethi Busnesau Bach".to_string(),
            "Small Business Bonus Scheme" => "Cynllun Bonws Busnesau Bach".to_string(),
            "Retail, Hospitality & Leisure Relief" => "Rhyddhad Manwerthu, Lletygarwch a Hamdden".to_string(),
            "Public Lavatory Relief" => "Rhyddhad Toiledau Cyhoeddus".to_string(),
            "Rural Settlement Relief" => "Rhyddhad Aneddiadau Gwledig".to_string(),
            "Improvement Relief" | "Improvement Relief (G-Factor)" => "Rhyddhad Gwelliannau (Ffactor-G)".to_string(),
            "Empty Property Low RV Exemption" => "Eithriad Eiddo Gwag Gwerth Isel".to_string(),
            "Empty Industrial Rate-Free Period (6 Months)" => "Cyfnod Di-dreth Eiddo Diwydiannol Gwag (6 Mis)".to_string(),
            "Empty Property Rate-Free Period (3 Months)" => "Cyfnod Di-dreth Eiddo Gwag (3 Mis)".to_string(),
            "Section 44A Part-Occupied Relief" => "Rhyddhad Rhan-Feddiant Adran 44A".to_string(),
            "Heat Network Relief" | "Heat Network Exemption" => "Rhyddhad Rhwydwaith Gwres".to_string(),
            "Anti-Avoidance Statutory Penalty" => "Cosb Statudol Gwrth-osgoi".to_string(),
            "Small Business Bonus Scheme Transitional Relief" => "Rhyddhad Trosiannol Cynllun Bonws Busnesau Bach".to_string(),
            "Supporting Small Business Relief" => "Rhyddhad Cefnogi Busnesau Bach".to_string(),
            "Welsh Transitional Relief" => "Rhyddhad Trosiannol Cymru".to_string(),
            "Welsh Food & Drink Hospitality Relief" => "Rhyddhad Ardrethi Lletygarwch Bwyd a Diod Cymru".to_string(),
            "Telecommunications Infrastructure Relief" => "Rhyddhad Seilwaith Telathrebu".to_string(),
            "Discretionary Charitable Rate Relief Top-Up" => "Ychwanegiad Dewisol at Ryddhad Elusennol".to_string(),
            "Renewable Energy Generation Relief" => "Rhyddhad Cynhyrchu Ynni Adnewyddadwy".to_string(),
            "Council Tax Transfer (Holiday Let Disqualification)" => "Trosglwyddo i'r Dreth Gyngor (Anghymhwyso Llety Gwyliau)".to_string(),
            other => other.to_string(),
        },
        Language::Sco => match name {
            "Mandatory Charitable Rate Relief" => "Mandatar Chairitable Rate Relief".to_string(),
            "Small Business Rate Relief" => "Small Business Rate Relief".to_string(),
            "Small Business Bonus Scheme" => "Small Business Bonus Scheme".to_string(),
            "Retail, Hospitality & Leisure Relief" => "Retail, Hospitality & Leisure Relief".to_string(),
            "Public Lavatory Relief" => "Public Lavatory Relief".to_string(),
            "Rural Settlement Relief" => "Kintra Settlement Relief".to_string(),
            "Improvement Relief" | "Improvement Relief (G-Factor)" => "Improvement Relief (G-Factor)".to_string(),
            "Empty Property Low RV Exemption" => "Tuim Hereditament Law RV Exemption".to_string(),
            "Empty Industrial Rate-Free Period (6 Months)" => "Tuim Industrial Rate-Free Period (6 Month)".to_string(),
            "Empty Property Rate-Free Period (3 Months)" => "Tuim Hereditament Rate-Free Period (3 Month)".to_string(),
            "Section 44A Part-Occupied Relief" => "Section 44A Pairt-Occupyit Relief".to_string(),
            "Heat Network Relief" | "Heat Network Exemption" => "Heat Netwark Relief".to_string(),
            "Anti-Avoidance Statutory Penalty" => "Anti-Avoidance Statutary Penalty".to_string(),
            "Small Business Bonus Scheme Transitional Relief" => "Small Business Bonus Scheme Transitional Relief".to_string(),
            "Supporting Small Business Relief" => "Supportin Smaa Business Relief".to_string(),
            "Welsh Transitional Relief" => "Welsh Transitional Relief".to_string(),
            "Welsh Food & Drink Hospitality Relief" => "Welsh Food & Drink Hospitality Relief".to_string(),
            "Telecommunications Infrastructure Relief" => "Telecommunications Infrastructure Relief".to_string(),
            "Discretionary Charitable Rate Relief Top-Up" => "Discretionary Chairitable Top-Up".to_string(),
            "Renewable Energy Generation Relief" => "Renewable Energy Generation Relief".to_string(),
            "Council Tax Transfer (Holiday Let Disqualification)" => "Council Tax Transfer (Holiday Let Disqualification)".to_string(),
            other => other.to_string(),
        },
        Language::Gd => match name {
            "Mandatory Charitable Rate Relief" => "Faochadh Ìrean Carthannais Èigneachail".to_string(),
            "Small Business Rate Relief" => "Faochadh Ìrean Gnìomhachais Bheaga".to_string(),
            "Small Business Bonus Scheme" => "Sgeama Bònas Ghnìomhachasan Beaga".to_string(),
            "Retail, Hospitality & Leisure Relief" => "Faochadh Mion-reic, Aoigheachd & Cur-seachad".to_string(),
            "Public Lavatory Relief" => "Faochadh Taighean-beaga Poblach".to_string(),
            "Rural Settlement Relief" => "Faochadh Tuineachaidh Dùthchail".to_string(),
            "Improvement Relief" | "Improvement Relief (G-Factor)" => "Faochadh Leasachaidh (Fachtor-G)".to_string(),
            "Empty Property Low RV Exemption" => "Saoradh Togalaich Fhalamh Luach Ìosal".to_string(),
            "Empty Industrial Rate-Free Period (6 Months)" => "Ùine Saor bho Ìrean Togalaich Ghnìomhachais Fhalamh (6 Mìosan)".to_string(),
            "Empty Property Rate-Free Period (3 Months)" => "Ùine Saor bho Ìrean Togalaich Fhalamh (3 Mìosan)".to_string(),
            "Section 44A Part-Occupied Relief" => "Faochadh Pàirt-ghabhailte Earrann 44A".to_string(),
            "Heat Network Relief" | "Heat Network Exemption" => "Faochadh Lìonra Teasa".to_string(),
            "Anti-Avoidance Statutory Penalty" => "Peanas Reachdail an-aghaidh Seachnadh".to_string(),
            "Small Business Bonus Scheme Transitional Relief" => "Faochadh Gluasaid Sgeama Bònas Ghnìomhachasan Beaga".to_string(),
            "Supporting Small Business Relief" => "Faochadh Taic do Ghnìomhachasan Beaga".to_string(),
            "Welsh Transitional Relief" => "Faochadh Gluasaid na Cuimrigh".to_string(),
            "Welsh Food & Drink Hospitality Relief" => "Faochadh Aoigheachd Bidhe is Deoch na Cuimrigh".to_string(),
            "Telecommunications Infrastructure Relief" => "Faochadh Bun-structair Tele-conaltraidh".to_string(),
            "Discretionary Charitable Rate Relief Top-Up" => "Barr-suim Roghainneil Faochadh Carthannais".to_string(),
            "Renewable Energy Generation Relief" => "Faochadh Gineadh Lùth Ath-nuadhachail".to_string(),
            "Council Tax Transfer (Holiday Let Disqualification)" => "Gluasad gu Cìs Comhairle (Dì-cheadachadh Togalaich Saor-làithean)".to_string(),
            other => other.to_string(),
        },
    }
}
