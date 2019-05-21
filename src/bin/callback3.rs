extern crate crossbeam;

type CB = FnMut(i32, i32) -> i32 + Send + Sync + 'static;

struct Processor {
    callback: Box<CB>,
}

impl Processor {
    fn new<CB>(cb: CB) -> Self
    where
        CB: FnMut(i32, i32) -> i32 + Send + Sync + 'static,
    {
        Processor {
            callback: Box::new(cb),
        }
    }

    fn process_event(&mut self) {
        let i = 5;
        (self.callback)(i, i * 2);
    }

    fn process_event_thread(&mut self) {
        let th = crossbeam::scope(|scope| {
            let i = 4;
            let j = 5;
            let jh = scope.spawn(move |_| (self.callback)(i, j * 2));
            jh.join().unwrap()
        })
        .unwrap();
        println!("{}", th * 100);
    }
}

fn main() {
    let k = 20;
    //let mut p = Processor {
    let mut p = Processor::new(move |i, j| {
        println!("{}, {}, {}", i, j, k);
        101
    });

    p.process_event();
    p.process_event_thread();
}
