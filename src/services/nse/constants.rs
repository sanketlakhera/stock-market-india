pub const MARKET_STATUS_URL: &str =
    "https://www1.nseindia.com//emerge/homepage/smeNormalMktStatus.json";
pub const INDICES_WATCH_URL: &str = "https://www1.nseindia.com/live_market/dynaContent/live_watch/stock_watch/liveIndexWatchData.json";
pub const SECTORS_LIST: &str = "https://www1.nseindia.com/homepage/peDetails.json";
pub const QUOTE_INFO_URL: &str = "https://www1.nseindia.com/live_market/dynaContent/live_watch/get_quote/ajaxGetQuoteJSON.jsp?series=EQ&symbol=";
pub const GET_QUOTE_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_watch/get_quote/GetQuote.jsp?symbol=";
pub const GAINERS_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_analysis/gainers/niftyGainers1.json";
pub const LOSERS_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_analysis/losers/niftyLosers1.json";
pub const ADVANCES_DECLINES_URL: &str =
    "https://www1.nseindia.com/common/json/indicesAdvanceDeclines.json";
pub const INDEX_STOCKS_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_watch/stock_watch/";
pub const INTRADAY_URL: &str = "https://www1.nseindia.com/charts/webtame/tame_intraday_getQuote_closing_redgreen.jsp?Segment=CM&Series=EQ&CDExpiryMonth=&FOExpiryMonth=&IRFExpiryMonth=&CDDate1=&CDDate2=&Template=tame_intraday_getQuote_closing_redgreen.jsp&CDSymbol=";
pub const SEARCH_URL: &str = "https://www1.nseindia.com/live_market/dynaContent/live_watch/get_quote/ajaxCompanySearch.jsp?search=";
pub const STOCK_OPTIONS_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_watch/get_quote/ajaxFOGetQuoteJSON.jsp";
pub const YEAR_HIGH_URL: &str =
    "https://www1.nseindia.com/products/dynaContent/equities/equities/json/online52NewHigh.json";
pub const YEAR_LOW_URL: &str =
    "https://www1.nseindia.com/products/dynaContent/equities/equities/json/online52NewLow.json";
pub const TOP_VALUE_URL: &str =
    "https://www1.nseindia.com/live_market/dynaContent/live_analysis/most_active/allTopValue1.json";
pub const TOP_VOLUME_URL: &str = "https://www1.nseindia.com/live_market/dynaContent/live_analysis/most_active/allTopVolume1.json";
pub const NEW_CHART_DATA_URL: &str =
    "https://www1.nseindia.com/ChartApp/install/charts/data/GetHistoricalNew.jsp";

// Default headers required for NSE requests
pub const DEFAULT_HEADERS: [(&str, &str); 3] = [
    ("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
    ("Accept", "*/*"),
    ("Accept-Language", "en-US,en;q=0.9"),
];
