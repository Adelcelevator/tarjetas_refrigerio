// @generated automatically by Diesel CLI.

pub mod administracion {
    diesel::table! {
        administracion.tbl_clientes (cli_id) {
            cli_id -> Int4,
            cli_ci -> Varchar,
            cli_nombre -> Varchar,
            cli_telefono -> Varchar,
            cli_direccion -> Varchar,
            cli_est -> Bool,
            usu_id_reg -> Int4,
            usu_id_ult_mod -> Int4,
            fecha_reg -> Timestamp,
            fecha_ult_mod -> Timestamp,
        }
    }

    diesel::table! {
        administracion.tbl_tipo_usuarios (tus_id) {
            tus_id -> Int4,
            tus_tipo -> Varchar,
            tus_est -> Bool,
        }
    }

    diesel::table! {
        administracion.tbl_usuarios (usu_id) {
            usu_id -> Int4,
            usu_nombre -> Varchar,
            usu_usuario -> Varchar,
            usu_contra -> Varchar,
            usu_est -> Varchar,
            tus_id -> Int4,
        }
    }

    diesel::joinable!(tbl_usuarios -> tbl_tipo_usuarios (tus_id));

    diesel::allow_tables_to_appear_in_same_query!(
        tbl_clientes,
        tbl_tipo_usuarios,
        tbl_usuarios,
    );
}
