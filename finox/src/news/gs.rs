extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// https://www.goldmansachs.com/insights/insights-articles.json

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub items: Vec<GSItem>,
    pub articles: Vec<GSArticle>,
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for t in self.articles.iter() {
            println!("{:#?}", t);
            recs.push(GSArticle::to_record(t));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GSItem {
    pub description: String,
    pub title: String,
    pub node_id: i64,
    pub url: String,
    pub featured_articles: Option<Vec<GSArticle>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GSArticle {
    pub has_video: bool,
    pub date: Option<String>,
    pub has_audio: bool,
    pub topics: Option<Vec<GSTopic>>,
    pub image_url: Option<String>,
    pub description: String,
    pub title: String,
    pub node_id: i64,
    pub url: String,
    pub series: Option<GSTopic>,
}

impl GSArticle {
    pub fn to_record(&self) -> Vec<String> {
        return vec![
            self.node_id.to_string(),
            self.date.clone().unwrap_or("".to_string()),
            self.title.to_string(),
            self.description.to_string(),
            self.has_video.to_string(),
            self.has_audio.to_string(),
        ];
        //rec.append(&mut lilmatcher_gstopic(self.series.clone()));
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GSTopic {
    pub title: String,
    pub node_id: i64,
    pub url: String,
}

impl GSTopic {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec![
            self.title.to_string(),
            self.node_id.to_string(),
            self.url.to_string(),
        ];

        return rec;
    }
}

