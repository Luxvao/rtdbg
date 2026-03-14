use crate::proc_utils::Vmas;

#[test]
fn test_vmas() {
    let vmas = Vmas::this().unwrap();

    println!("{:?}", vmas);
}
