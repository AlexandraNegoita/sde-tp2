// Note this useful idiom: importing names from outer (for mod tests) scope.
use crate::Structure;

#[test]
fn test_add() {
    let mut vector=Structure::new();
    vector.add(9);
    assert_eq!(vector.v, vec![9]);
    vector.add(8);
    vector.add(0);
    assert_eq!(vector.v, vec![0,8,9]);
    vector.add(-5);
    assert_eq!(vector.v, vec![-5,0,8,9]);
    vector.add(-2);
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
    vector.add(6);
    vector.add(9);
    vector.remove(3);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(2);
    assert_eq!(vector.v,vec![6,9]);
    vector.remove(6);
    vector.remove(9);
    assert_eq!(vector.v,vec![]);
}