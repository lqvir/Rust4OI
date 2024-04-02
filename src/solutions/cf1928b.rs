use super::scanner::Scanner;

pub fn works(){
    let mut scan = Scanner::default();
    let mut round = scan.next::<u64>();

    while(round != 0){
        round -= 1;
        let n = scan.next::<u64>();
        let mut a: Vec<u64> = Vec::new();
        let mut b: Vec<u64> = Vec::new();
        for _ in 0..n{
            a.push(scan.next::<u64>());
        }

        a.sort();
        b.push(1);
        for i in 1..n{
            if(a[i as usize]!=a[i as usize - 1]){
                b.push(a[i as usize]-a[0] + 1);
            }
        }
        let lenb = b.len();
        let mut maxlen = 0;
        for i in 0..lenb{
            let start = b[i];
            let st = b.binary_search(&start).ok().unwrap();
            let ed = match b.binary_search(&(start+n)){
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            
            if maxlen < ed- st {
                maxlen = ed - st ; 
            }
        }
        println!("{}",maxlen);
    } 
}


#[test]
fn binary_search_test(){
    let testvec = vec![1,2,3,4,5];
    let res = testvec.binary_search(&2);
    let out = res.ok().unwrap();
    print!("{}",out);

}