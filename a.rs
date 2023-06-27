
use slack_hook;
use slack_hook::{Slack, PayloadBuilder};
use scraper::{Html, Selector};
use tabled::{Tabled};
use chrono::prelude::*;
// use tabled::tables::IterTable;

#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

#[tokio::main]
async fn main() {
    // let local: DateTime<Local> = Local::now(); 
    let mut names = Vec::new();
        names.push("");
        names.push("cinoss");
        names.push("long29031997");
        names.push("phiquan1606");
        names.push("lam3082004");
        names.push("cuongal");
        names.push("imakite");
        names.push("tuht97");
        names.push("mikamokedo");
        names.push("anhchile");
        names.push("lovehary90");
        names.push("tainh274");
        names.push("phieulong97");

        // send slack
    let mut sort1 = Vec::new();
    let mut sort2 = Vec::new();
    let mut sort3 = Vec::new();
    for i in 0..names.len(){
        let mut kq: Vec<String> = call_data_racer_api(names[i].to_string()).await;
        // kq.push("\n");
        // kq.remove(kq.len());
        if kq[1] == " "{
            for _ in 1..10{
                kq[0].push(' ');
            }
            if kq[12] != "NaN" {
                kq[12] = "NaN".to_string();
                kq[13] = "%".to_string();
            }
            sort1.push(kq);
        }
        else if kq[1].len() == 5{
            sort2.push(kq);
        }
        else {
            sort3.push(kq);
        }
    }

    // let mut kq: Vec<String> = call_data_racer_api(names[3].to_string()).await;
    // sort1.push(kq);
    // println!("{:?}", sort1);

    sort2.sort_by(|a, b| (a[1].parse::<f32>().unwrap()).partial_cmp(&b[1].parse::<f32>().unwrap()).unwrap());
    let mut str: String = "".to_string();
    for word in sort3.into_iter().rev(){
        let string = word.into_iter().collect::<String>();
        str.push_str(&format!("{}", string));
    }
    for word in sort2.into_iter().rev(){
        let string = word.into_iter().collect::<String>();
        str.push_str(&format!("{}", string));
    }
    for word in sort1.into_iter().rev(){
        let string = word.into_iter().collect::<String>();
        str.push_str(&format!("{}", string));
    } 

    let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
    let p = PayloadBuilder::new()
      .text(str)
      .channel("#testing")
      .username("My Bot")
      .icon_emoji(":chart_with_upwards_trend:")
      .build()
      .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("OK"),
        Err(x) => println!("ERR: {:?}",x)
    }
}

async fn call_data_racer_api(mut name: String) -> Vec<String>{
    let mut url = "https://typeracerdata.com/profile?username=".to_string();
    url.push_str(&name.clone());
    let res = reqwest::get(url).await.unwrap();
    
    let local: DateTime<Local> = Local::now(); 
    let name_len = name.len();
    let mut data = Vec::new();
    let mut a = 0;
    let mut check = Vec::new();

    let doc_body = Html::parse_document(&res.text().await.unwrap());
    let kq = Selector::parse(".r").unwrap();
    
    let mut size = name_len;
    if name == "mikamokedo"{
            size += 6;
    }
    else if name == "lovehary90"{
        size += 3;
    }
    else if name == "long29031997"{
        size += 8;
    }
    else if name == "cinoss"{
        size -= 1;
    }
    else if name == "tainh274"{
        size += 2;
    }
    else if name == "phiquan1606" || name == "lam3082004"{
        size += 7;
    }
    else if name == "cuongal"{
        size += 1;
    }
    else if name == "phieulong97"{
        size += 5;
    }
    if name_len != 0{
        loop{
            // data.push(" ".to_string());
            name.push(' ');
            size += 1;
            if size == 38{
                break;
            }
        }
    }
    data.push(name.clone());

    for kq in doc_body.select(&kq){
        let kqs = kq.text().collect::<Vec<_>>();
        // print!("{:#?}", kqs[0]);
        check.push(kqs[0])
    }
    let mut dem = 0;
    let mut c = 0;
    let mut a1: f32= 0.0;
    let mut a2: f32 = 0.0;
    for i in check{
        if dem ==  20{
            break;
        }
        if dem == 19{
            // a1.push(i.to_string());
            a1 = (i.to_string()).parse::<f32>().unwrap();
        }
        if dem == 13{
            a2 = (i.to_string()).parse::<f32>().unwrap();
        }
        if dem != 14 && dem != 16 && dem != 17 && dem != 18 && dem != 19{
            if a == 1{
                // if i == "May 2023"{
                //     break;
                // }
                if c == 1{
                    for _ in 1..12{
                        data.push(" ".to_string());
                    }
                }
                c = 1;
                data.push(i.to_string());
            }
            if i == "June 2023"{
                a = 1;
            }
        }
        dem +=1 ;
    }
    if name_len != 0{
        for _ in 1..12{
            data.push(" ".to_string());
        }
        let  zz = ((((a2/a1) - 1 as f32) as f32) * 100 as f32).to_string();
        data.push(zz[..3].to_string());
        data.push("%".to_string());
    }
    if data.len() as usize == 1 && name_len != 0{
        // for _ in 1..5{
        //     data.push(" ".to_string());
        // }
        data.push("N/A".to_string());
    }
    // data.push(data.len().to_string());
    if name_len == 0{
        data.push(local.to_string()[..19].to_string());
        // data.push("\nName      	                       Average	 Best	Races	Wins	Win %".to_string()); 
        data.push("\nName      	                       Average  	Races       Increase %".to_string()); 
        // data.push("\nName      	                       Average  	Races       ".to_string()); 
    }
    data.push("\n".to_string());
    data
}
