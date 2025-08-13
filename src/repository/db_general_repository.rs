// @generated automatically by Diesel CLI.

pub mod db_general {
    diesel::table! {
        db_general.tbl_error (error_id) {
            error_id -> Int8,
            #[max_length = 100]
            error_proceso -> Nullable<Varchar>,
            #[max_length = 100]
            error_funcion -> Nullable<Varchar>,
            error_error -> Nullable<Text>,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
        }
    }

    diesel::table! {
        db_general.tbl_log (log_id) {
            log_id -> Int8,
            #[max_length = 100]
            log_funcion -> Nullable<Varchar>,
            #[max_length = 100]
            log_accion -> Nullable<Varchar>,
            log_log -> Nullable<Text>,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
        }
    }

    diesel::table! {
        db_general.tbl_parametro_cab (id_parametro_cab) {
            id_parametro_cab -> Nullable<Int4>,
            #[max_length = 2000]
            nombre_cabecera -> Varchar,
            #[max_length = 2000]
            descripcion_parametro -> Nullable<Varchar>,
            #[max_length = 100]
            estado -> Varchar,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
            fe_modificacion -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        db_general.tbl_parametro_det (id_parametro_det) {
            id_parametro_det -> Nullable<Int4>,
            id_parametro_cab -> Int4,
            #[max_length = 100]
            nombre -> Varchar,
            #[max_length = 1000]
            parametro_descripcion -> Nullable<Varchar>,
            parametro_valor -> Text,
            #[max_length = 100]
            estado -> Varchar,
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
            usr_modificacion -> Nullable<Varchar>,
            fe_modificacion -> Nullable<Timestamp>,
        }
    }

    diesel::joinable!(tbl_parametro_det -> tbl_parametro_cab (id_parametro_cab));

    diesel::allow_tables_to_appear_in_same_query!(
        tbl_error,
        tbl_log,
        tbl_parametro_cab,
        tbl_parametro_det,
    );
}
