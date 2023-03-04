use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    events: Option<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    top_match: Vec<TopMatch>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopMatch {
    event_sales_status: super::info::EventSalesStatus,
    timezone: String,
    id: String,
    tickets_url: String,
    primary_organizer_id: String,
    start_date: String,
    end_time: String,
    ticket_availability: Option<TicketAvailability>,
    end_date: String,
    eventbrite_event_id: String,
    start_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketAvailability {
    is_sold_out: bool,
    is_free: bool,
    has_available_tickets: bool,
    minimum_ticket_price: TicketPriceInfo,
    has_tickets: bool,
    maximum_ticket_price: TicketPriceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketPriceInfo {
    currency: String,
    major_value: String,
    value: i32,
    display: String,
}

impl Search {
    pub fn new() -> Search {
        return Search { events: None };
    }

    pub async fn fetch(&self) {
        #[derive(Serialize, Deserialize, Debug)]
        struct PayLoad {
            event_search: EventSearch,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct EventSearch {
            q: String,
            dates: String,
            dedup: bool,
            places: Vec<String>,
            page: i32,
            page_size: i32,
            online_events_only: bool,
            #[serde(rename = "expand.destination_event")]
            expand_destination_event: Vec<String>,
            include_promoted_events_for: IncludePromotedEventsFor,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct IncludePromotedEventsFor {
            interface: String,
            request_source: String,
        }

        // {
        // "event_search": {
        // "q": "coding meetups",
        // "dates": "current_future",
        // "dedup": true,
        // "places": [
        // "101735835"
        // ],
        // "page": 1,
        // "page_size": 20,
        // "online_events_only": false,
        // "include_promoted_events_for": {
        // "interface": "search",
        // "request_source": "web"
        // }
        // },
        // "expand.destination_event": [
        // "primary_venue",
        // "image",
        // "ticket_availability",
        // "saves",
        // "event_sales_status",
        // "primary_organizer",
        // "public_collections"
        // ],
        // "debug_experiment_overrides": {
        // "search_exp_3": "A"
        // }
        // }
        let url = "https://www.eventbrite.ca/api/v3/destination/search/";
        let client = reqwest::Client::new();
        let body = PayLoad {
            event_search: EventSearch {
                q: "coding meetups".to_string(),
                dates: "current_future".to_string(),
                dedup: true,
                places: vec!["101735835".to_string()],
                page: 1,
                page_size: 20,
                online_events_only: false,
                expand_destination_event: vec![
                    "primary_venue".to_string(),
                    "image".to_string(),
                    "ticket_availability".to_string(),
                    "saves".to_string(),
                    "event_sales_status".to_string(),
                    "primary_organizer".to_string(),
                    "public_collections".to_string(),
                ],
                include_promoted_events_for: IncludePromotedEventsFor {
                    interface: "search".to_string(),
                    request_source: "web".to_string(),
                },
            },
        };

        let body = r#"{"event_search":{"q":"coding meetups","dates":"current_future","dedup":true,"places":["101735835"],"page":1,"page_size":20,"online_events_only":false,"include_promoted_events_for":{"interface":"search","request_source":"web"}},"expand.destination_event":["primary_venue","image","ticket_availability","saves","event_sales_status","primary_organizer","public_collections"],"debug_experiment_overrides":{"search_exp_3":"A"}}"#;

        println!("body: {}", to_string_pretty(&body).unwrap());
        let mut headers = HeaderMap::new();
        headers.insert(
            "referer",
            HeaderValue::from_static(
                "https://www.eventbrite.ca/d/canada--toronto/coding-meetups/?page=1",
            ),
        );
        headers.insert("cookie", HeaderValue::from_static(r#"G=v%3D2%26i%3D53667589-8561-456a-9056-ff021cc9f445%26a%3D10e2%26s%3D3c91c4a781e20bb8b26292f11257c452f85e9ce5; eblang=lo%3Den_CA%26la%3Den-ca; AS=551feafa-1cf7-480f-990f-3abc358677b5; csrftoken=aff71306b84c11ed852d1f998e7209b9; mgrefby="https://www.google.com/"; mgref=refsites; _gid=GA1.2.1640365782.1677687601; ebGAClientId=1585740576.1677687601; _gcl_au=1.1.526030219.1677687603; _scid=23f0784d-27e4-4194-99d4-df45aeda2ad8; _tt_enable_cookie=1; _ttp=19j5jfRJrfUI5HSYC3rUZRisnJ9; _pin_unauth=dWlkPU9EazRPR0UzTXpVdE5XWTNaQzAwWkRKbUxUbG1OVGd0TlRBek4yWTNOVE16WlRobQ; _sctr=1|1677646800000; hubspotutk=9a5508a87b8c873482aaf38f850d8c36; __hssrc=1; mgaff518737516877=ebdssbdestsearch; ajs_user_id=null; ajs_group_id=null; ajs_anonymous_id=%22ac147c92-22a9-44e9-a10a-6809acdbea97%22; _hp2_props.1404198904=%7B%7D; ln_or=eyI5NDQzNiI6ImQifQ%3D%3D; SS=AE3DLHSjQyybml188Kc8oOcTpsbb1e3sZg; _hp2_id.1404198904=%7B%22userId%22%3A%225330432376296632%22%2C%22pageviewId%22%3A%227845173215556958%22%2C%22sessionId%22%3A%221285814541628496%22%2C%22identity%22%3Anull%2C%22trackerVersion%22%3A%224.0%22%7D; _hp2_ses_props.1404198904=%7B%22ts%22%3A1677890949632%2C%22d%22%3A%22www.eventbrite.ca%22%2C%22h%22%3A%22%2Fd%2Fcanada--toronto%2Fcoding-meetups%2F%22%2C%22q%22%3A%22%3Fpage%3D1%22%7D; _ga_TQVES5V6SH=GS1.1.1677890943.8.1.1677890954.0.0.0; _ga=GA1.1.1585740576.1677687601; __hstc=58577909.9a5508a87b8c873482aaf38f850d8c36.1677687605873.1677815975964.1677890954675.8; __hssc=58577909.1.1677890954675; SP=AGQgbblq75_pMzrX9rhUnIk90lUfC3TwT9xKUBJZQWTV21z3cAen19q_-GB1Vdeti7-IAFf7qa8DjQcj3R7afJheML5sal0YXc8Fpv2EObeaXimrXgu2IMhfl3mF_52FBKQVGBXTKvMOKAA6LlovXljOeLLijNneGZzUdJOs28ouDdtA-KwWWyW9PedU1Jtg1taLRPAua0qaF4WSXVVDcRKl019yvWOhJB9nPcOgiZw8xYGEgudJvRE; _dd_s=rum=0"#));
        // &expire=1677891905534  // removed expiration date. Put back if necessary

        headers.insert(
            "x-csrftoken",
            HeaderValue::from_static("aff71306b84c11ed852d1f998e7209b9"),
        );

        let res = client
            .post(url)
            .json(&body)
            .headers(headers)
            .send()
            .await
            .unwrap();
        println!("Status: {}", res.text().await.unwrap());
    }
}
