use super::scanner::Scanner;


#[test]
pub fn works(){
    let mut scan = Scanner::default();
    let round = scan.next::<usize>();
    for _ in 0..round{
        let n = scan.next::<usize>();
        let grid1 = scan.next::<String>(); 
        let grid2 = scan.next::<String>(); 

        let mut grids = [grid1.chars(),grid2.chars()];
        
        let mut rowid = 0 as usize;
        grids[0].next();
        for pos in 1..n{

            

            rowid ^= 1;

        }


    }
}
