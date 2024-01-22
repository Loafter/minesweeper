fn main() {
    let mut count:u64=0;
    for e  in 1..=150u64 {
        println!("curent e={} total iter={}",e,count);
        for a in 1..=150u64 {
            for b in a..=150u64 {     
                for c in b..=150u64 {
                    for d in c..=150u64 {
                        if a.pow(5)+b.pow(5)+c.pow(5)+d.pow(5)==e.pow(5){
                            println!("a={} b={} c={} d={} e={} sum={}",a,b,c,d,e,a+b+c+d+e);
                            return;
                        }
                        count+=1;
                    }
                }
            }
        }
    }
}
