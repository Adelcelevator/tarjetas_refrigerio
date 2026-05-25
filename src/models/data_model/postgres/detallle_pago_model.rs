use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::postgres::db_cobros_repository::db_cobros::tbl_detalle_pago;

#[derive(Queryable,Insertable,Serialize,Deserialize
,Debug,Clone)]
#[diesel(table_name=tbl_detalle_pago )]
pub struct DetallePago{
    pub detalle_pago_id : Option<i32>,
    pub tipo_pago_id : Option<i32>,
    pub pago_id : Option<i32>,
    pub detalle_pago_valor : rust_decimal::Decimal,
    pub estado : String,
    pub usr_creacion : String,
    pub fe_creacion : NaiveDateTime,
    pub usr_modificacion : Option<String>,
    pub fe_modificacion : Option<NaiveDateTime>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DetallePagoReq{
    pub tipo_pago_id: i32,
    pub detalle_pago_valor:f32,
}