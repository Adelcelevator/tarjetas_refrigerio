// @generated automatically by Diesel CLI.

pub mod db_cobros {
    diesel::table! {
        db_cobros.tbl_detalle_pago (detalle_pago_id) {
            detalle_pago_id -> Nullable<Int4>,
            tipo_pago_id -> Nullable<Int4>,
            pago_id -> Nullable<Int4>,
            detalle_pago_valor -> Float8,
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
        db_cobros.tbl_historial_pago (historial_pago_id) {
            historial_pago_id -> Nullable<Int4>,
            pago_id -> Nullable<Int4>,
            #[max_length = 4000]
            historial_detalle -> Nullable<Varchar>,
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
        db_cobros.tbl_pago (pago_id) {
            pago_id -> Nullable<Int4>,
            per_id -> Nullable<Int4>,
            pago_valor_total -> Float8,
            #[max_length = 400]
            pago_observacion -> Nullable<Varchar>,
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
        db_cobros.tbl_tipo_pago (tipo_pago_id) {
            tipo_pago_id -> Nullable<Int4>,
            #[max_length = 100]
            pago_descricion -> Nullable<Varchar>,
            #[max_length = 100]
            estado -> Varchar,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
            fe_modificacion -> Nullable<Timestamptz>,
        }
    }

    diesel::joinable!(tbl_detalle_pago -> tbl_pago (pago_id));
    diesel::joinable!(tbl_detalle_pago -> tbl_tipo_pago (tipo_pago_id));
    diesel::joinable!(tbl_historial_pago -> tbl_pago (pago_id));

    diesel::allow_tables_to_appear_in_same_query!(
        tbl_detalle_pago,
        tbl_historial_pago,
        tbl_pago,
        tbl_tipo_pago,
    );
}
