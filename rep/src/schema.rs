// @generated automatically by Diesel CLI.

diesel::table! {
    AD_ADRESSE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        STAAT -> Nullable<Text>,
        PLZ -> Nullable<Text>,
        ORT -> Text,
        STRASSE -> Nullable<Text>,
        HAUSNR -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    AD_PERSON (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        TYP -> Integer,
        GESCHLECHT -> Text,
        GEBURT -> Nullable<Date>,
        GEBURTK -> Integer,
        ANREDE -> Integer,
        FANREDE -> Integer,
        NAME1 -> Text,
        NAME2 -> Nullable<Text>,
        PRAEDIKAT -> Nullable<Text>,
        VORNAME -> Nullable<Text>,
        TITEL -> Nullable<Text>,
        PERSON_STATUS -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    AD_SITZ (MANDANT_NR, PERSON_UID, REIHENFOLGE, UID) {
        MANDANT_NR -> Integer,
        PERSON_UID -> Text,
        REIHENFOLGE -> Integer,
        UID -> Text,
        TYP -> Integer,
        NAME -> Text,
        ADRESSE_UID -> Nullable<Text>,
        TELEFON -> Nullable<Text>,
        FAX -> Nullable<Text>,
        MOBIL -> Nullable<Text>,
        EMAIL -> Nullable<Text>,
        HOMEPAGE -> Nullable<Text>,
        POSTFACH -> Nullable<Text>,
        BEMERKUNG -> Nullable<Text>,
        SITZ_STATUS -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    AG_DIALOG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        API -> Text,
        DATUM -> Date,
        NR -> Integer,
        URL -> Nullable<Text>,
        FRAGE -> Nullable<Text>,
        ANTWORT -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    BENUTZER (MANDANT_NR, BENUTZER_ID) {
        MANDANT_NR -> Integer,
        BENUTZER_ID -> Text,
        PASSWORT -> Nullable<Text>,
        BERECHTIGUNG -> Integer,
        AKT_PERIODE -> Integer,
        PERSON_NR -> Integer,
        GEBURT -> Nullable<Date>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    BYTE_DATEN (MANDANT_NR, TYP, UID, LFD_NR) {
        MANDANT_NR -> Integer,
        TYP -> Text,
        UID -> Text,
        LFD_NR -> Integer,
        METADATEN -> Nullable<Text>,
        BYTES -> Nullable<Binary>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    FZ_BUCH (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        AUTOR_UID -> Text,
        SERIE_UID -> Text,
        SERIENNUMMER -> Integer,
        TITEL -> Text,
        UNTERTITEL -> Nullable<Text>,
        SEITEN -> Integer,
        SPRACHE_NR -> Integer,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    FZ_BUCHAUTOR (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        NAME -> Text,
        VORNAME -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    FZ_BUCHSERIE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        NAME -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    FZ_BUCHSTATUS (MANDANT_NR, BUCH_UID) {
        MANDANT_NR -> Integer,
        BUCH_UID -> Text,
        IST_BESITZ -> Bool,
        LESEDATUM -> Nullable<Date>,
        HOERDATUM -> Nullable<Date>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    FZ_FAHRRAD (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEZEICHNUNG -> Text,
        TYP -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    FZ_FAHRRADSTAND (MANDANT_NR, FAHRRAD_UID, DATUM, NR) {
        MANDANT_NR -> Integer,
        FAHRRAD_UID -> Text,
        DATUM -> Date,
        NR -> Integer,
        ZAEHLER_KM -> Double,
        PERIODE_KM -> Double,
        PERIODE_SCHNITT -> Double,
        BESCHREIBUNG -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    FZ_NOTIZ (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        THEMA -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HH_BILANZ (MANDANT_NR, PERIODE, KZ, KONTO_UID) {
        MANDANT_NR -> Integer,
        PERIODE -> Integer,
        KZ -> Text,
        KONTO_UID -> Text,
        SH -> Text,
        BETRAG -> Double,
        ESH -> Text,
        EBETRAG -> Double,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HH_BUCHUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        SOLL_VALUTA -> Date,
        HABEN_VALUTA -> Date,
        KZ -> Nullable<Text>,
        BETRAG -> Double,
        EBETRAG -> Double,
        SOLL_KONTO_UID -> Text,
        HABEN_KONTO_UID -> Text,
        BTEXT -> Text,
        BELEG_NR -> Nullable<Text>,
        BELEG_DATUM -> Date,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HH_EREIGNIS (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        KZ -> Nullable<Text>,
        SOLL_KONTO_UID -> Text,
        HABEN_KONTO_UID -> Text,
        BEZEICHNUNG -> Text,
        ETEXT -> Text,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HH_KONTO (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        SORTIERUNG -> Text,
        ART -> Text,
        KZ -> Nullable<Text>,
        NAME -> Text,
        GUELTIG_VON -> Nullable<Date>,
        GUELTIG_BIS -> Nullable<Date>,
        PERIODE_VON -> Integer,
        PERIODE_BIS -> Integer,
        BETRAG -> Double,
        EBETRAG -> Double,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HH_PERIODE (MANDANT_NR, NR) {
        MANDANT_NR -> Integer,
        NR -> Integer,
        DATUM_VON -> Date,
        DATUM_BIS -> Date,
        ART -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_BEHANDLUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        PATIENT_UID -> Text,
        DATUM -> Date,
        DAUER -> Double,
        BESCHREIBUNG -> Text,
        DIAGNOSE -> Nullable<Text>,
        BETRAG -> Double,
        LEISTUNG_UID -> Text,
        RECHNUNG_UID -> Nullable<Text>,
        STATUS_UID -> Text,
        MITTEL -> Nullable<Text>,
        POTENZ -> Nullable<Text>,
        DOSIERUNG -> Nullable<Text>,
        VERORDNUNG -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_BEHANDLUNG_LEISTUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEHANDLUNG_UID -> Text,
        LEISTUNG_UID -> Text,
        DAUER -> Double,
        PARAMETER -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_LEISTUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        ZIFFER -> Text,
        ZIFFER_ALT -> Text,
        BESCHREIBUNG_FETT -> Text,
        BESCHREIBUNG -> Text,
        FAKTOR -> Double,
        FESTBETRAG -> Double,
        FRAGEN -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_LEISTUNGSGRUPPE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEZEICHNUNG -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_PATIENT (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        NAME1 -> Text,
        VORNAME -> Nullable<Text>,
        GESCHLECHT -> Text,
        GEBURT -> Nullable<Date>,
        ADRESSE1 -> Nullable<Text>,
        ADRESSE2 -> Nullable<Text>,
        ADRESSE3 -> Nullable<Text>,
        RECHNUNG_PATIENT_UID -> Nullable<Text>,
        STATUS -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_RECHNUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        RECHNUNGSNUMMER -> Text,
        DATUM -> Date,
        PATIENT_UID -> Text,
        BETRAG -> Double,
        DIAGNOSE -> Text,
        STATUS_UID -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    HP_STATUS (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        STATUS -> Text,
        BESCHREIBUNG -> Text,
        SORTIERUNG -> Integer,
        STANDARD -> Integer,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    MA_MANDANT (NR) {
        NR -> Integer,
        BESCHREIBUNG -> Text,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    MA_PARAMETER (MANDANT_NR, SCHLUESSEL) {
        MANDANT_NR -> Integer,
        SCHLUESSEL -> Text,
        WERT -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    SB_EREIGNIS (MANDANT_NR, PERSON_UID, FAMILIE_UID, TYP, TAG1, MONAT1, JAHR1) {
        MANDANT_NR -> Integer,
        PERSON_UID -> Text,
        FAMILIE_UID -> Text,
        TYP -> Text,
        TAG1 -> Integer,
        MONAT1 -> Integer,
        JAHR1 -> Integer,
        TAG2 -> Integer,
        MONAT2 -> Integer,
        JAHR2 -> Integer,
        DATUM_TYP -> Text,
        ORT -> Nullable<Text>,
        BEMERKUNG -> Nullable<Text>,
        QUELLE_UID -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    SB_FAMILIE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        MANN_UID -> Nullable<Text>,
        FRAU_UID -> Nullable<Text>,
        STATUS1 -> Integer,
        STATUS2 -> Integer,
        STATUS3 -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    SB_KIND (MANDANT_NR, FAMILIE_UID, KIND_UID) {
        MANDANT_NR -> Integer,
        FAMILIE_UID -> Text,
        KIND_UID -> Text,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    SB_PERSON (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        NAME -> Text,
        VORNAME -> Nullable<Text>,
        GEBURTSNAME -> Nullable<Text>,
        GESCHLECHT -> Nullable<Text>,
        TITEL -> Nullable<Text>,
        KONFESSION -> Nullable<Text>,
        BEMERKUNG -> Nullable<Text>,
        QUELLE_UID -> Nullable<Text>,
        STATUS1 -> Integer,
        STATUS2 -> Integer,
        STATUS3 -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    SB_QUELLE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BESCHREIBUNG -> Text,
        ZITAT -> Nullable<Text>,
        BEMERKUNG -> Nullable<Text>,
        AUTOR -> Text,
        STATUS1 -> Integer,
        STATUS2 -> Integer,
        STATUS3 -> Integer,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    TB_EINTRAG (MANDANT_NR, DATUM) {
        MANDANT_NR -> Integer,
        DATUM -> Date,
        EINTRAG -> Text,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
        REPLIKATION_UID -> Nullable<Text>,
    }
}

diesel::table! {
    TB_EINTRAG_ORT (MANDANT_NR, ORT_UID, DATUM_VON, DATUM_BIS) {
        MANDANT_NR -> Integer,
        ORT_UID -> Text,
        DATUM_VON -> Date,
        DATUM_BIS -> Date,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    TB_ORT (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEZEICHNUNG -> Nullable<Text>,
        BREITE -> Double,
        LAENGE -> Double,
        HOEHE -> Double,
        ZEITZONE -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    TB_WETTER (MANDANT_NR, DATUM, ORT_UID, API) {
        MANDANT_NR -> Integer,
        DATUM -> Date,
        ORT_UID -> Text,
        API -> Text,
        WERTE -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    WP_ANLAGE (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        WERTPAPIER_UID -> Text,
        BEZEICHNUNG -> Text,
        PARAMETER -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    WP_BUCHUNG (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        WERTPAPIER_UID -> Text,
        ANLAGE_UID -> Text,
        DATUM -> Date,
        ZAHLUNGSBETRAG -> Double,
        RABATTBETRAG -> Double,
        ANTEILE -> Double,
        ZINSEN -> Double,
        BTEXT -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    WP_KONFIGURATION (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEZEICHNUNG -> Text,
        PARAMETER -> Text,
        STATUS -> Text,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    WP_STAND (MANDANT_NR, WERTPAPIER_UID, DATUM) {
        MANDANT_NR -> Integer,
        WERTPAPIER_UID -> Text,
        DATUM -> Date,
        STUECKPREIS -> Double,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::table! {
    WP_WERTPAPIER (MANDANT_NR, UID) {
        MANDANT_NR -> Integer,
        UID -> Text,
        BEZEICHNUNG -> Text,
        KUERZEL -> Text,
        PARAMETER -> Nullable<Text>,
        DATENQUELLE -> Text,
        STATUS -> Text,
        RELATION_UID -> Nullable<Text>,
        NOTIZ -> Nullable<Text>,
        ANGELEGT_VON -> Nullable<Text>,
        ANGELEGT_AM -> Nullable<Timestamp>,
        GEAENDERT_VON -> Nullable<Text>,
        GEAENDERT_AM -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    AD_ADRESSE,
    AD_PERSON,
    AD_SITZ,
    AG_DIALOG,
    BENUTZER,
    BYTE_DATEN,
    FZ_BUCH,
    FZ_BUCHAUTOR,
    FZ_BUCHSERIE,
    FZ_BUCHSTATUS,
    FZ_FAHRRAD,
    FZ_FAHRRADSTAND,
    FZ_NOTIZ,
    HH_BILANZ,
    HH_BUCHUNG,
    HH_EREIGNIS,
    HH_KONTO,
    HH_PERIODE,
    HP_BEHANDLUNG,
    HP_BEHANDLUNG_LEISTUNG,
    HP_LEISTUNG,
    HP_LEISTUNGSGRUPPE,
    HP_PATIENT,
    HP_RECHNUNG,
    HP_STATUS,
    MA_MANDANT,
    MA_PARAMETER,
    SB_EREIGNIS,
    SB_FAMILIE,
    SB_KIND,
    SB_PERSON,
    SB_QUELLE,
    TB_EINTRAG,
    TB_EINTRAG_ORT,
    TB_ORT,
    TB_WETTER,
    WP_ANLAGE,
    WP_BUCHUNG,
    WP_KONFIGURATION,
    WP_STAND,
    WP_WERTPAPIER,
);
