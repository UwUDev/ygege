use crate::nostr::NostrClient;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use sysinfo::{Pid, System};

#[get("/bench")]
pub async fn bench_mark(
    nostr: web::Data<NostrClient>,
    req_data: HttpRequest,
) -> HttpResponse {
    let query = req_data.query_string();
    let qs = QString::from(query);
    let search_count: usize = qs.get("search_count").unwrap_or("1").parse().unwrap_or(1);

    let stream = async_stream::stream! {
        yield Ok::<_, actix_web::Error>(web::Bytes::from("bench_name,metric,value\n"));

        let mut sys = System::new_all();
        sys.refresh_all();
        let pid = Pid::from_u32(std::process::id());

        if let Some(process) = sys.process(pid) {
            let mem = process.memory();
            let line = format!("{},{},{}\n", "memory_usage", "b", mem);
            yield Ok(web::Bytes::from(line));
        }

        for _ in 0..search_count {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            let mut sys = System::new_all();
            sys.refresh_all();
            let pid = Pid::from_u32(std::process::id());

            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            sys.refresh_all();

            let cpu_before = sys.process(pid).map(|p| p.cpu_usage()).unwrap_or(0.0);

            let start = chrono::Utc::now();
            let _search = search(
                &nostr,
                "Vaiana",
                None,
                Some(Sort::Seed),
                Some(Order::Ascending),
                None,
            ).await;
            let duration = chrono::Utc::now().signed_duration_since(start);

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            sys.refresh_all();
            let cpu_after = sys.process(pid).map(|p| p.cpu_usage()).unwrap_or(0.0);

            yield Ok(web::Bytes::from(format!("{},{},{}\n", "search_torrent", "ns", duration.num_nanoseconds().unwrap_or(0))));
            yield Ok(web::Bytes::from(format!("{},{},{}\n", "search_cpu_before", "percent", cpu_before)));
            yield Ok(web::Bytes::from(format!("{},{},{}\n", "search_cpu_after", "percent", cpu_after)));
            yield Ok(web::Bytes::from(format!("{},{},{}\n", "search_cpu_delta", "percent", (cpu_after - cpu_before).abs())));
        }
    };

    HttpResponse::Ok()
        .content_type("text/csv")
        .append_header(("Content-Disposition", "attachment; filename=\"benchmark.csv\""))
        .streaming(stream)
}
