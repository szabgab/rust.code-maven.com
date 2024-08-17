fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    let autocommit = unsafe { sqlite::ffi::sqlite3_get_autocommit(connection.as_raw()) };
    println!("{autocommit}");

    assert_eq!(1, unsafe {
        sqlite::ffi::sqlite3_get_autocommit(connection.as_raw())
    });

    connection.execute("BEGIN").unwrap();
    assert_eq!(0, unsafe {
        sqlite::ffi::sqlite3_get_autocommit(connection.as_raw())
    });

    connection.execute("END").unwrap();
    assert_eq!(1, unsafe {
        sqlite::ffi::sqlite3_get_autocommit(connection.as_raw())
    });

    connection.execute("BEGIN").unwrap();
    assert_eq!(0, unsafe {
        sqlite::ffi::sqlite3_get_autocommit(connection.as_raw())
    });

    connection.execute("ROLLBACK").unwrap();
    assert_eq!(1, unsafe {
        sqlite::ffi::sqlite3_get_autocommit(connection.as_raw())
    });
}
