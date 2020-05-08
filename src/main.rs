extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use clap::{Arg, App};
use std::net::{TcpListener, TcpStream};
use chrono::{DateTime, NaiveDateTime, Utc, Local};


struct Scan {
    id: Option<i64>,
    time : Option<DateTime<Local>>,
    name: Option<String>,
}

struct OpenPort {
    id: Option<i64>,
    port: Option<u16>,
    scan_id: Option<i64>,
}

fn main() -> Result<()>{

    let dt = Local::now();
    let database_name = dt.to_string().replace(" ", "-");

    let matches = App::new("Port scanner")
        .version("0.1.0")
        .author("Lorne Barnaby <lorne123barnaby@gmail.com>")
        .about("Scans for open or unavailable ports")
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("The name of the database"))
        .arg(Arg::with_name("scan")
            .short("s")
            .long("scan")
            .takes_value(true)
            .help("The name of the scan"))
        .get_matches();

    let scan_name = matches.value_of("name").unwrap_or(database_name.as_str());
    let scan_dir = scan_name.to_string() + ".db";




    let conn= Connection::open(scan_dir.as_str())?;

    let scan_title = matches.value_of("scan").unwrap_or(database_name.as_str());

    create_or_connect(&conn);
    let scanid = create_scan(scan_title.to_string(), &conn).unwrap();


    let open_ports = (1..65535).filter(|port| try_port(*port));
    for port in open_ports{
        insert_port(port, &conn, scanid);
    }

    Ok(())

}

fn create_or_connect(conn: &Connection) -> Result<()>{

    conn.execute(
        "create table if not exists Scan (
             id integer primary key AUTOINCREMENT,
             name text not null unique,
             time datetime
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists OpenPort (
             id integer primary key AUTOINCREMENT,
             port integer not null,
             scan_id integer not null references scan(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn insert_port(port: u16, conn: &Connection, scanid: i64) -> Result<()>{
    match conn.execute(
        "insert into OpenPort(port, scan_id) VALUES (?1, ?2)",
        &[port as i64, scanid]
    ){
        Err(E) => panic!("{}",E),
        _ => {}
    };

    Ok(())
}

fn create_scan(name: String, conn : &Connection) -> Result<i64>{
    let mut scan: Scan = Scan{
        id: None,
        time: None,
        name : Some(name),
    };

    conn.execute(
        "INSERT INTO Scan (name, time) values (?1, datetime('now'))",
        &[&scan.name],
    )?;

    let scan_id = conn.last_insert_rowid();
    scan.id = Option::from(scan_id);

    Ok(scan_id)
}


fn try_port(port: u16) -> bool{
    match TcpStream::connect(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
