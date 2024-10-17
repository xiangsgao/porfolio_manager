struct Pie {
    id: i32,
    pie_name: String
}


struct Asset{
    id: i32,
    parent_pie: i32,
    asset_type: String,
    asset_price: i32,
    asset_ticker: String
}

struct Porfolio {
    cash: i32,
    invested: i32,
}





async fn get_database(){

}