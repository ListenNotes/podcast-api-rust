macro_rules! b {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

mod mock {
    use serde_json::json;

    fn client<'a>() -> podcast_api::Client<'a> {
        podcast_api::Client::new(reqwest::Client::new(), None)
    }

    #[test]
    fn search() {
        let response = b!(client().search(&json!({
            "q": "dummy",
            "sort_by_date": 1
        })))
        .unwrap();
        assert!(response.is_object());
        assert!(response["results"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn typeahead() {
        let response = b!(client().typeahead(&json!({
            "q": "dummy",
            "show_podcasts": 1
        })))
        .unwrap();
        assert!(response.is_object());
        assert!(response["terms"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn best_podcasts() {
        let response = b!(client().best_podcasts(&json!({
            "genre_id": 23,
        })))
        .unwrap();
        assert!(response.is_object());
        assert!(response["total"].as_i64().unwrap() > 0);
    }

    #[test]
    fn podcast() {
        let response = b!(client().podcast("dummy_id", &json!({}))).unwrap();
        assert!(response.is_object());
        assert!(response["episodes"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn podcasts() {
        let response = b!(client().podcasts(&json!({
            "ids": "996,777,888,1000"
        })))
        .unwrap();
        assert!(response.is_object());
        assert!(response["podcasts"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn episode() {
        let response = b!(client().episode("dummy_id", &json!({}))).unwrap();
        assert!(response.is_object());
        assert!(
            response["podcast"].as_object().unwrap()["rss"]
                .as_str()
                .unwrap()
                .len()
                > 0
        );
    }

    #[test]
    fn episodes() {
        let response = b!(client().episodes(&json!({
            "ids": "996,777,888,1000"
        })))
        .unwrap();
        assert!(response.is_object());
        assert!(response["episodes"].as_array().unwrap().len() > 0);
    }
}
