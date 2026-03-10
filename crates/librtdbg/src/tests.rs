use crate::proc_utils::Vma;

#[test]
fn test_vmas() {
    let vmas = Vma::this().unwrap();

    println!("{:?}", vmas);
}
