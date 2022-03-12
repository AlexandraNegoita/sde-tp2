// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::Structure;

#[test]
fn test_add() {
    let mut vector=Structure::new();
    vector.add(9);
    assert_eq!(vector.v, vec![9]);
    vector.add(8);
    assert_eq!(vector.v, vec![8,9]);
    vector.add(0);
    assert_eq!(vector.v, vec![0,8,9]);
    vector.add(-5);
    assert_eq!(vector.v, vec![-5,0,8,9]);
    vector.add(-2);
    assert_eq!(vector.v, vec![-5,-2,0,8,9]);
    vector.add(-1);
    assert_eq!(vector.v, vec![-5,-2,-1,0,8,9]);
    vector.add(0);
    assert_eq!(vector.v, vec![-5,-2,-1,0,8,9]);
    vector.add(4);
    assert_eq!(vector.v, vec![-5,-2,-1,0,4,8,9]);
}

#[test]
fn test_remove() {
    let mut vector=Structure::new();
    vector.add(3);
    assert_eq!(vector.v,vec![3]);
    vector.add(6);
    assert_eq!(vector.v,vec![3,6]);
    vector.add(9);
    assert_eq!(vector.v,vec![3,6,9]);
    vector.remove(3);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(2);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(6);
    assert_eq!(vector.v,vec![9]);
    vector.remove(9);
    assert_eq!(vector.v,vec![]);
    vector.remove(12);
    assert_eq!(vector.v,vec![]);
}

#[test]
fn prime_numbers(){
    let mut vector=Structure::new();
    vector.add(3);
    assert_eq!(vector.v,vec![3]);
    vector.add(6);
    assert_eq!(vector.v,vec![3,6]);
    vector.add(9);
    assert_eq!(vector.v,vec![3,6,9]);
    assert_eq!(vector.prime_numbers(),vec![3]);
    vector.remove(3);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(2);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(6);
    assert_eq!(vector.v,vec![9]);
    vector.remove(9);
    assert_eq!(vector.v,vec![]);
    vector.add(9);
    assert_eq!(vector.v, vec![9]);
    vector.add(8);
    assert_eq!(vector.v, vec![8,9]);
    vector.add(0);
    assert_eq!(vector.v, vec![0,8,9]);
    vector.add(-5);
    assert_eq!(vector.v, vec![-5,0,8,9]);
    vector.add(-2);
    assert_eq!(vector.v, vec![-5,-2,0,8,9]);
    vector.add(-1);
    assert_eq!(vector.v, vec![-5,-2,-1,0,8,9]);
    vector.add(0);
    assert_eq!(vector.v, vec![-5,-2,-1,0,8,9]);
    vector.add(4);
    assert_eq!(vector.v, vec![-5,-2,-1,0,4,8,9]);
    assert_eq!(vector.prime_numbers(),vec![]);
    vector.remove(9);
    assert_eq!(vector.v, vec![-5,-2,-1,0,4,8]);
    vector.remove(8);
    assert_eq!(vector.v, vec![-5,-2,-1,0,4]);
    vector.remove(0);
    assert_eq!(vector.v, vec![-5,-2,-1,4]);
    vector.remove(-5);
    assert_eq!(vector.v, vec![-2,-1,4]);
    vector.remove(-2);
    assert_eq!(vector.v, vec![-1,4]);
    vector.remove(-1);
    assert_eq!(vector.v, vec![4]);
    vector.remove(0);
    assert_eq!(vector.v, vec![4]);
    vector.remove(4);
    assert_eq!(vector.v, vec![]);
    vector.add(11);
    assert_eq!(vector.v, vec![11]);
    vector.add(16);
    assert_eq!(vector.v, vec![11,16]);
    vector.add(0);
    assert_eq!(vector.v, vec![0,11,16]);
    vector.add(-25);
    assert_eq!(vector.v, vec![-25,0,11,16]);
    vector.add(1);
    assert_eq!(vector.v, vec![-25,0,1,11,16]);
    vector.add(-18);
    assert_eq!(vector.v, vec![-25,-18,0,1,11,16]);
    assert_eq!(vector.prime_numbers(),vec![11]);
}