use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponseMarket
{
    pub page: i16,
    pub total_pages: i16,
    pub total_count: i16,
    pub rows_per_page: i16,
    pub interval : ApiResponseMarketInterval,
    pub rows: Vec<MarketItem>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseMarketInterval
{
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketItem {
    pub nameid: i32,
    pub name: String,
    pub jname: String,
    pub value_buy: i32,
    pub value_sell: i32,
    pub r#type: i32,
    pub subtype: i32,
    pub sex: i32,
    pub equip: i32,
    pub weight: i32,
    pub atk: i32,
    pub r#def: i32,
    pub r#range: i32,
    pub slot: i32,
    pub view_sprite: i32,
    pub elv: i32,
    pub wlv: i32,
    pub view_id: i32,
    pub matk: i32,
    pub elvmax: i32,
    pub delay: i32,
    pub flag_trade: i32,
    pub flag_refine: i32,
    pub flag_buystore: i32,
    pub id: i32,
    pub account_id: i32,
    pub char_id: i32,
    pub char_name: String,
    pub shop_name: String,
    pub mapname: String,
    pub map_x: i32,
    pub map_y: i32,
    pub job: i32,
    pub clone_id: i32,
    pub idx: i32,
    pub price: i32,
    pub identify: i32,
    pub refine: i32,
    pub attribute: i32,
    pub unique_id: u64,
    pub bonus_time: i32,
    pub data: MarketItemData,
    pub opts: Vec<String>,
    pub cards: Vec<MarketItemCard>,
    pub cards_sulfix: Vec<MarketItemCardSufix>,
    pub amount: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketItemData {
    pub icon: String,
    pub collection: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketItemCard {
    pub card_id: i32,
    pub card_name: String,
    pub icon: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketItemCardSufix {
    pub sulfix: String,
    pub count: i32,
}