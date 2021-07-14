use crate::{
    entities::app::{AppDetail, ColorStyle, CreateAppParams, UpdateAppParams},
    impl_sql_binary_expression,
    sql_tables::workspace::Workspace,
};
use diesel::sql_types::Binary;
use flowy_database::schema::app_table;
use flowy_infra::{timestamp, uuid};
use serde::{Deserialize, Serialize, __private::TryFrom};
use std::convert::TryInto;

#[derive(
    PartialEq,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Queryable,
    Identifiable,
    Insertable,
    Associations,
)]
#[belongs_to(Workspace, foreign_key = "workspace_id")]
#[table_name = "app_table"]
#[serde(tag = "type")]
pub(crate) struct App {
    pub id: String,
    pub workspace_id: String, // equal to #[belongs_to(Workspace, foreign_key = "workspace_id")].
    pub name: String,
    pub desc: String,
    pub color_style: ColorStyleCol,
    pub last_view_id: Option<String>,
    pub modified_time: i64,
    pub create_time: i64,
    pub version: i64,
}

impl App {
    pub fn new(params: CreateAppParams) -> Self {
        let app_id = uuid();
        let time = timestamp();
        Self {
            id: app_id,
            workspace_id: params.workspace_id,
            name: params.name,
            desc: params.desc,
            color_style: params.color_style.into(),
            last_view_id: None,
            modified_time: time,
            create_time: time,
            version: 0,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default, FromSqlRow, AsExpression)]
#[sql_type = "Binary"]
pub(crate) struct ColorStyleCol {
    pub(crate) theme_color: String,
}

impl std::convert::From<ColorStyle> for ColorStyleCol {
    fn from(s: ColorStyle) -> Self {
        Self {
            theme_color: s.theme_color,
        }
    }
}

impl std::convert::TryInto<Vec<u8>> for &ColorStyleCol {
    type Error = String;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(self).map_err(|e| format!("{:?}", e))
    }
}

impl std::convert::TryFrom<&[u8]> for ColorStyleCol {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value).map_err(|e| format!("{:?}", e))
    }
}

impl_sql_binary_expression!(ColorStyleCol);

#[derive(AsChangeset, Identifiable, Default, Debug)]
#[table_name = "app_table"]
pub struct AppChangeset {
    pub id: String,
    pub workspace_id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl AppChangeset {
    pub fn new(params: UpdateAppParams) -> Self {
        AppChangeset {
            id: params.app_id,
            workspace_id: params.workspace_id,
            name: params.name,
            desc: params.desc,
        }
    }
}

impl std::convert::Into<AppDetail> for App {
    fn into(self) -> AppDetail {
        AppDetail {
            id: self.id,
            workspace_id: self.workspace_id,
            name: self.name,
            desc: self.desc,
        }
    }
}