// @generated automatically by Diesel CLI.

pub mod db_tarjetas {
    diesel::table! {
        db_tarjetas.tbl_comprobante_representado (comprobante_representado_id) {
            comprobante_representado_id -> Nullable<Int4>,
            repre_id -> Int4,
            comp_id -> Int4,
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
        db_tarjetas.tbl_comprobantes (comp_id) {
            comp_id -> Nullable<Int4>,
            #[max_length = 100]
            comp_numero -> Varchar,
            per_id -> Nullable<Int4>,
            #[max_length = 30]
            estado -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
            comp_valor -> Numeric,
            comp_path_fisico -> Text,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_historial_comprobante (hist_comp_id) {
            hist_comp_id -> Nullable<Int8>,
            comp_id -> Nullable<Int4>,
            #[max_length = 100]
            estado -> Nullable<Varchar>,
            hist_comp_hist -> Nullable<Text>,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamp,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_historial_tarjeta (histo_tar_id) {
            histo_tar_id -> Nullable<Int4>,
            tar_id -> Nullable<Int4>,
            repre_id -> Nullable<Int4>,
            #[max_length = 1000]
            histo_tar_observacion -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            tar_saldo -> Nullable<Numeric>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_persona (per_id) {
            per_id -> Nullable<Int4>,
            #[max_length = 100]
            per_nombre -> Varchar,
            #[max_length = 100]
            per_identificacion -> Varchar,
            #[max_length = 20]
            per_telefono -> Nullable<Varchar>,
            #[max_length = 100]
            per_direccion -> Nullable<Varchar>,
            #[max_length = 10]
            estado -> Nullable<Varchar>,
            fe_creacion -> Timestamptz,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamptz>,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
            per_saldo -> Numeric,
            unidad_id -> Nullable<Int4>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_representante_representado (repre_id) {
            repre_id -> Nullable<Int4>,
            repsentante_id -> Nullable<Int4>,
            repsentado_id -> Nullable<Int4>,
            #[max_length = 50]
            estado -> Nullable<Varchar>,
            #[max_length = 100]
            usu_creacion -> Nullable<Varchar>,
            fe_creacion -> Nullable<Timestamp>,
            #[max_length = 100]
            usu_modificacion -> Nullable<Varchar>,
            fe_modificacion -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_roles (rol_id) {
            rol_id -> Nullable<Int4>,
            #[max_length = 100]
            rol_rol -> Nullable<Varchar>,
            #[max_length = 500]
            rol_descripcion -> Nullable<Varchar>,
            #[max_length = 100]
            estado -> Nullable<Varchar>,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            usr_modificacion -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_roles_usuario (rol_usu_id) {
            rol_usu_id -> Nullable<Int4>,
            rol_id -> Nullable<Int4>,
            usu_id -> Nullable<Int4>,
            #[max_length = 10]
            estado -> Nullable<Varchar>,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usu_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            #[max_length = 100]
            usu_modificacion -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_tarjeta (tar_id) {
            tar_id -> Nullable<Int4>,
            tar_saldo -> Numeric,
            comp_id -> Nullable<Int4>,
            per_id -> Nullable<Int4>,
            #[max_length = 10]
            estado -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_tarjeta_representado (tar_per_id) {
            tar_per_id -> Nullable<Int4>,
            repre_id -> Nullable<Int4>,
            tar_id -> Nullable<Int4>,
            #[max_length = 10]
            estado -> Varchar,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_unidades (unidad_id) {
            unidad_id -> Nullable<Int4>,
            #[max_length = 200]
            unidad_nombre -> Varchar,
            #[max_length = 100]
            estado -> Varchar,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_creacion -> Timestamptz,
            #[max_length = 100]
            usr_modificacion -> Nullable<Varchar>,
            fe_modificacion -> Nullable<Timestamptz>,
        }
    }

    diesel::table! {
        db_tarjetas.tbl_usuario (usu_id) {
            usu_id -> Nullable<Int4>,
            #[max_length = 100]
            usu_usuario -> Nullable<Varchar>,
            #[max_length = 100]
            usu_contra -> Nullable<Varchar>,
            #[max_length = 10]
            estado -> Nullable<Varchar>,
            fe_creacion -> Timestamp,
            #[max_length = 100]
            usr_creacion -> Varchar,
            fe_modificacion -> Nullable<Timestamp>,
            usr_modificacion -> Nullable<Varchar>,
            per_id -> Nullable<Int4>,
        }
    }

    diesel::joinable!(tbl_comprobante_representado -> tbl_comprobantes (comp_id));
    diesel::joinable!(tbl_comprobante_representado -> tbl_representante_representado (repre_id));
    diesel::joinable!(tbl_comprobantes -> tbl_persona (per_id));
    diesel::joinable!(tbl_historial_comprobante -> tbl_comprobantes (comp_id));
    diesel::joinable!(tbl_historial_tarjeta -> tbl_representante_representado (repre_id));
    diesel::joinable!(tbl_historial_tarjeta -> tbl_tarjeta (tar_id));
    diesel::joinable!(tbl_persona -> tbl_unidades (unidad_id));
    diesel::joinable!(tbl_roles_usuario -> tbl_roles (rol_id));
    diesel::joinable!(tbl_roles_usuario -> tbl_usuario (usu_id));
    diesel::joinable!(tbl_tarjeta -> tbl_comprobantes (comp_id));
    diesel::joinable!(tbl_tarjeta -> tbl_persona (per_id));
    diesel::joinable!(tbl_tarjeta_representado -> tbl_representante_representado (repre_id));
    diesel::joinable!(tbl_tarjeta_representado -> tbl_tarjeta (tar_id));
    diesel::joinable!(tbl_usuario -> tbl_persona (per_id));

    diesel::allow_tables_to_appear_in_same_query!(
        tbl_comprobante_representado,
        tbl_comprobantes,
        tbl_historial_comprobante,
        tbl_historial_tarjeta,
        tbl_persona,
        tbl_representante_representado,
        tbl_roles,
        tbl_roles_usuario,
        tbl_tarjeta,
        tbl_tarjeta_representado,
        tbl_unidades,
        tbl_usuario,
    );
}
