CARGO_FEATURE_PURE=1 cargo xwin build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/alictf.pdb ./
cp target/x86_64-pc-windows-msvc/release/alictf.exe ./
zip -r release.zip alictf.exe alictf.pdb assets/
