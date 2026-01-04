use crate::auth::login;
use crate::config::Config;
use crate::domain::get_ygg_domain;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use sysinfo::{Pid, System};
use wreq::Client;

#[get("/bench")]
pub async fn bench_mark(
    data: web::Data<Client>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> HttpResponse {
    let query = req_data.query_string();
    let qs = QString::from(query);
    let search_count: usize = qs.get("search_count").unwrap_or("1").parse().unwrap_or(1);
    let login_count: usize = qs.get("login_count").unwrap_or("1").parse().unwrap_or(1);
    let domain_count: usize = qs.get("domain_count").unwrap_or("1").parse().unwrap_or(1);

    let stream = async_stream::stream! {
        yield Ok::<_, actix_web::Error>(web::Bytes::from("bench_name,metric,value\n"));

        // Current memory usage
        let mut sys = System::new_all();
        sys.refresh_all();
        let pid = Pid::from_u32(std::process::id());

        if let Some(process) = sys.process(pid) {
            let mem = process.memory();
            let line = format!("{},{},{}\n",
                "memory_usage",
                "b",
                mem
            );

            yield Ok(web::Bytes::from(line));
        }


        // Search benchmark with CPU usage
        for _ in 0..search_count {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            // Measure CPU before search
            let mut sys = System::new_all();
            sys.refresh_all();
            let pid = Pid::from_u32(std::process::id());

            // Wait a bit and refresh to get accurate CPU reading
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            sys.refresh_all();

            let cpu_before = if let Some(process) = sys.process(pid) {
                process.cpu_usage()
            } else {
                0.0
            };

            let start = chrono::Utc::now();
            let _search = search(
                &data,
                &config,
                Some("Vaiana"),
                None,
                None,
                None,
                Some(Sort::Seed),
                Some(Order::Ascending),
                None,
            ).await;
            let duration = chrono::Utc::now().signed_duration_since(start);

            // Measure CPU after search
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            sys.refresh_all();
            let cpu_after = if let Some(process) = sys.process(pid) {
                process.cpu_usage()
            } else {
                0.0
            };

            // Output time duration
            let line = format!("{},{},{}\n",
                "search_torrent",
                "ns",
                duration.num_nanoseconds().unwrap_or(0)
            );
            yield Ok(web::Bytes::from(line));

            // Output CPU usage before
            let line = format!("{},{},{}\n",
                "search_cpu_before",
                "percent",
                cpu_before
            );
            yield Ok(web::Bytes::from(line));

            // Output CPU usage after
            let line = format!("{},{},{}\n",
                "search_cpu_after",
                "percent",
                cpu_after
            );
            yield Ok(web::Bytes::from(line));

            // Output CPU delta
            let line = format!("{},{},{}\n",
                "search_cpu_delta",
                "percent",
                (cpu_after - cpu_before).abs()
            );
            yield Ok(web::Bytes::from(line));
        }

        // Login benchmark without session restore
        for _ in 0..login_count {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let start = chrono::Utc::now();
            let _login = login(&config, true).await;
            let duration = chrono::Utc::now().signed_duration_since(start);
            let line = format!("{},{},{}\n",
                "user_login_no_restore",
                "ns",
                duration.num_nanoseconds().unwrap_or(0)
            );
            yield Ok(web::Bytes::from(line));
        }

        // Login benchmark with session restore
        for _ in 0..login_count {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let start = chrono::Utc::now();
            let _login = login(&config, true).await;
            let duration = chrono::Utc::now().signed_duration_since(start);
            let line = format!("{},{},{}\n",
                "user_login_with_restore",
                "ns",
                duration.num_nanoseconds().unwrap_or(0)
            );
            yield Ok(web::Bytes::from(line));
        }

        // Resolve domain benchmark
        for _ in 0..domain_count {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let start = chrono::Utc::now();
            let _domain = get_ygg_domain().await;
            let duration = chrono::Utc::now().signed_duration_since(start);
            let line = format!("{},{},{}\n",
                "resolve_domain",
                "ns",
                duration.num_nanoseconds().unwrap_or(0)
            );
            yield Ok(web::Bytes::from(line));
        }
    };

    HttpResponse::Ok()
        .content_type("text/csv")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"benchmark.csv\"",
        ))
        .streaming(stream)
}
