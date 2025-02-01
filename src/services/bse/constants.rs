// BSE API endpoints
pub const BSE_BASE_URL: &str = "https://api.bseindia.com/BseIndiaAPI/api";
pub const INDICES_URL: &str = "https://www.bseindia.com/Sensex/IndicesWatch_New.aspx";
pub const INDEX_INFO_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/MktRGainerLoserData/w?GLtype=";
pub const INDEX_STOCKS_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/StockReachGraph/w?scripcode=";
pub const STOCK_INFO_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/StockReachGraph/w?scripcode=";
pub const GAINERS_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/MktRGainerLoserData/w?GLtype=gainer";
pub const LOSERS_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/MktRGainerLoserData/w?GLtype=loser";
pub const TOP_TURNOVER_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/MktRGainerLoserData/w?GLtype=turnover";
pub const STOCK_CHART_URL: &str =
    "https://api.bseindia.com/BseIndiaAPI/api/StockReachGraph/w?scripcode=";

// Default headers required for BSE requests
pub const DEFAULT_HEADERS: [(&str, &str); 4] = [
    ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
    ("Accept", "application/json, text/plain, */*"),
    ("Accept-Language", "en-US,en;q=0.9"),
    ("Origin", "https://www.bseindia.com"),
];
