svd2rust --target riscv --max-cluster-size -i k210.svd
Remove-Item -Path "src" -Recurse
form -i lib.rs -o src/ 
Remove-Item lib.rs
cargo fmt
