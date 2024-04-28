use crate::models::{ApiResponseMarket, MarketItem};


pub async fn send_request(page : i16) -> ApiResponseMarket
{
    let url : String = String::from("http://api.ragnatales.com.br/market"); 
     
    let recaptcha = "03AFcWeA7SSEEYawB11veQ5Nswqw5uyTSVzCUDd6apFSbpiv3Op_CAwIHxdZeoGLmwfsoFpSqxty9ZNeaEO_5wdJG1S8Xof3w2sSSfpMzLTVsn2CNZQcz_MjbeY3DqkxfAZk7QGq2tLqu-pP2XW5wR3nZMzdgNdLfp4kN-e3Eodirz6Uabod2U-Fqm0yL16xtBghaH4oQeUvMlaEiY67lD1mML0rIxeorrAPCKnE28e55Ow_cX4R8A6pjPp5adxXHFOo1zfgxWn8HuLyavVE5Pr6i3sql0IIMMLur2m8eoBJ49s1xm44FcSy3GA_QnKGGANhTtP6h4IajUH42FHPPCjxbi1asjgb9h5hB8uMKBVM1o6pL49WgvIFXJOCqbS6KTsEVEcBwtQbm0awk6GQMaILgYEPFQME5BpAy4QtCjCPu9RJgLPQvo3tv_rASTZIoafBh7IZP-oF4D21-1Azus5RVLrsuYko9obaNhmSPFBhT4MIP5_y9jNMtYvVsxkcRnK0MenDsSeGgig8XELudVzuFbBWkZOH4lZuSz5TClaRrunOw4Jb8xcFQkOJAqDwjnLWZnl_qIkGTtV7d5feGte9JoSFQShVYoPgxwW1Vutxc68P-giDxWOtHKtS779E4hDA72fD6tpAHlad9_k16zMOOZ47mfeUbrBprE-cVvW-TObrL_56MXZuf1ba2PDQTxvDeMrfqLl3OPvby9yQeqWYRl6-0sSZ-FPM1zxYPa44jYig_J6ZeqC55hdcEdsOp-cl_Tcz2M6Y3QfflvRO2wIfgLdMm9M0N5zA";
    let rows_per_page = 30;
    
    let query_string = format!("?page={page}&rows_per_page={rows_per_page}&filters=%7B%22query%22:%22%22%7D&g_recaptcha_response={recaptcha}");
    let address = format!("{url}{query_string}");
    
    let client = reqwest::Client::new();

    let res = match client.get(address)
            .header("Authorization", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NzM2NTAsImVtYWlsIjoibGVvbmFyZG9wYWdvdHRvMEBnbWFpbC5jb20iLCJnZW5lcmF0ZWRBdCI6MTcxMzY1MTEyOS4zMTM0MDIsImlwQWRkcmVzcyI6IjE3Ny4xMzguMTU0LjIwMSIsInVzZXJBZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEyMy4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEyMy4wIn0.805XwowcoCxiiQ6bwREUKzJEyTOCcUlQIov8JkIoSBI")
            .header("Cookie", "PHPSESSID=6853f8b2d7b94e8483661dfea334b1e3")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0")
            .header("Origin", "https://ragnatales.com.br")
            .header("Referer", "https://ragnatales.com.br/")
            .header("Host", "api.ragnatales.com.br")
            .header("Connection", "keep-alive")
            .header("Accept", "application/json, text/plain, */*")
            .send()
            .await {
        Ok(it) => it,
        Err(err) => panic!("Error ao enviar request"),
    };


    let content = match res.json::<ApiResponseMarket>().await {
        Ok(data) => data,
        Err(e) => panic!("Error deserializing JSON response: {}", e)
    };

    return content;
}