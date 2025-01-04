pub fn enumerate(http_client:&Client, target: &str) -> Result<Vec<Subdomain>,
    Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    //clear and dedup results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry|)
}
