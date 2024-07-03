macro_rules! prt {
    ($var:expr) => {
        println!(
            "{:10} {:10} {:p} {:15?}",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
        );
    };
}

fn main() {
    let len = 100_000_000;
    {
        let start = std::time::Instant::now();
        let mut text = String::new();
        
        for i in 0..len {
            text.push_str("x");
            let _t = String::from("x"); 
            if i == 0 {
                prt!(text);
            }
        }
        prt!(text);
        println!("{:?}", start.elapsed());
        println!("len: {}", text.len());    
    }

    println!();

    {
        let start = std::time::Instant::now();
        let mut text = String::with_capacity(len);
        for i in 0..len {            
            text.push_str("x");
            let _t = String::from("x");
            if i == 0 {
                prt!(text);
            }
        }
        prt!(text);
        println!("{:?}", start.elapsed());
        println!("len: {}", text.len());
    }

}
