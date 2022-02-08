use crate::structs;

const API_URL_ROOT: &str = "https://hacker-news.firebaseio.com/v0";

pub async fn get_item(id: i32) -> Result<structs::Item, reqwest::Error> {
    let item = reqwest::get(format!("{}/item/{}.json", API_URL_ROOT, id))
        .await?
        .json::<structs::Item>()
        .await?;

    Ok(item)
}

pub async fn get_user(id: &str) -> Result<structs::User, reqwest::Error> {
    let user = reqwest::get(format!("{}/user/{}.json", API_URL_ROOT, id))
        .await?
        .json::<structs::User>()
        .await?;

    Ok(user)
}

pub async fn get_maxitem() -> Result<i32, reqwest::Error> {
    let maxitem = reqwest::get(format!("{}/maxitem.json", API_URL_ROOT))
        .await?
        .text()
        .await?
        .parse::<i32>().unwrap();

    Ok(maxitem)
}

pub async fn get_topstories() -> Result<Vec<i32>, reqwest::Error> {
    let topstories = reqwest::get(format!("{}/topstories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(topstories)
}

pub async fn get_newstories() -> Result<Vec<i32>, reqwest::Error> {
    let newstories = reqwest::get(format!("{}/newstories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(newstories)
}

pub async fn get_beststories() -> Result<Vec<i32>, reqwest::Error> {
    let beststories = reqwest::get(format!("{}/beststories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(beststories)
}

pub async fn get_askstories() -> Result<Vec<i32>, reqwest::Error> {
    let askstories = reqwest::get(format!("{}/askstories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(askstories)
}

pub async fn get_showstories() -> Result<Vec<i32>, reqwest::Error> {
    let showstories = reqwest::get(format!("{}/showstories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(showstories)
}

pub async fn get_jobstories() -> Result<Vec<i32>, reqwest::Error> {
    let jobstories = reqwest::get(format!("{}/jobstories.json", API_URL_ROOT))
        .await?
        .json::<Vec<i32>>()
        .await?;

    Ok(jobstories)
}

pub async fn get_updates() -> Result<structs::Updates, reqwest::Error> {
    let updates = reqwest::get(format!("{}/jobstories.json", API_URL_ROOT))
        .await?
        .json::<structs::Updates>()
        .await?;

    Ok(updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_item_post_test() {
        let item = get_item(8863).await.unwrap();
        assert_eq!(item.by.unwrap(), "dhouston");
        assert_eq!(item.id, 8863);
        assert_eq!(item.time.unwrap(), 1175714200);
        assert_eq!(item.title.unwrap(), "My YC app: Dropbox - Throw away your USB drive");
        assert_eq!(item.item_type.unwrap(), "story");
        assert_eq!(item.url.unwrap(), "http://www.getdropbox.com/u/2/screencast.html");
    }

    #[tokio::test]
    async fn get_item_comment_test() {
        let item = get_item(2921983).await.unwrap();
        assert_eq!(item.by.unwrap(), "norvig");
        assert_eq!(item.id, 2921983);
        assert_eq!(item.parent.unwrap(), 2921506);
        assert_eq!(item.text.unwrap(), "Aw shucks, guys ... you make me blush with your compliments.<p>Tell you what, Ill make a deal: I'll keep writing if you keep reading. K?");
        assert_eq!(item.time.unwrap(), 1314211127);
        assert_eq!(item.item_type.unwrap(), "comment");
    }

    #[tokio::test]
    async fn get_item_job_test() {
        let item = get_item(192327).await.unwrap();
        assert_eq!(item.by.unwrap(), "justin");
        assert_eq!(item.id, 192327);
        assert_eq!(item.time.unwrap(), 1210981217);
        assert_eq!(item.title.unwrap(), "Justin.tv is looking for a Lead Flash Engineer!");
        assert_eq!(item.item_type.unwrap(), "job");
        assert_eq!(item.url.unwrap(), "");
        assert_eq!(item.text.unwrap(), "Justin.tv is the biggest live video site online. We serve hundreds of thousands of video streams a day, and have supported up to 50k live concurrent viewers. Our site is growing every week, and we just added a 10 gbps line to our colo. Our unique visitors are up 900% since January.<p>There are a lot of pieces that fit together to make Justin.tv work: our video cluster, IRC server, our web app, and our monitoring and search services, to name a few. A lot of our website is dependent on Flash, and we're looking for talented Flash Engineers who know AS2 and AS3 very well who want to be leaders in the development of our Flash.<p>Responsibilities<p><pre><code>    * Contribute to product design and implementation discussions\n    * Implement projects from the idea phase to production\n    * Test and iterate code before and after production release \n</code></pre>\nQualifications<p><pre><code>    * You should know AS2, AS3, and maybe a little be of Flex.\n    * Experience building web applications.\n    * A strong desire to work on website with passionate users and ideas for how to improve it.\n    * Experience hacking video streams, python, Twisted or rails all a plus.\n</code></pre>\nWhile we're growing rapidly, Justin.tv is still a small, technology focused company, built by hackers for hackers. Seven of our ten person team are engineers or designers. We believe in rapid development, and push out new code releases every week. We're based in a beautiful office in the SOMA district of SF, one block from the caltrain station. If you want a fun job hacking on code that will touch a lot of people, JTV is for you.<p>Note: You must be physically present in SF to work for JTV. Completing the technical problem at <a href=\"http://www.justin.tv/problems/bml\" rel=\"nofollow\">http://www.justin.tv/problems/bml</a> will go a long way with us. Cheers!")
    }
}