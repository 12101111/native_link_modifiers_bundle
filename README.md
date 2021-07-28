```shell
export RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=ld.bfd"
cargo run --target=x86_64-unknown-linux-musl
```

```
   Compiling cc v1.0.69
   Compiling foo v0.1.0 (/tmp/musl-link)
error: linking with `ld.bfd` failed: exit status: 1
  |
  = note: "ld.bfd" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o" "--as-needed" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Bstatic" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib" "--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-lfoo" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844" "--gc-sections" "-static" "-pie" "--no-dynamic-linker" "-z" "text" "-zrelro" "-znow" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
  = note: ld.bfd: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a(foo.o): in function `bar':
          /tmp/musl-link/foo.c:3: undefined reference to `printf'
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `foo` due to previous error
```

Rust version:

```
> rustc -Vv
rustc 1.56.0-nightly (2faabf579 2021-07-27)
binary: rustc
commit-hash: 2faabf579323f5252329264cc53ba9ff803429a3
commit-date: 2021-07-27
host: x86_64-unknown-linux-musl
release: 1.56.0-nightly
LLVM version: 12.0.1
```

# verbose log

```shell
export RUSTC_LOG=rustc_codegen_ssa::back::link=info
export RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=ld.bfd -Clink-arg=--verbose"
cargo run --target=x86_64-unknown-linux-musl -v
```

```
   Compiling cc v1.0.69
     Running `rustc --crate-name cc --edition=2018 /home/han/.cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.69/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1d671f13a31d6bec -C extra-filename=-1d671f13a31d6bec --out-dir /tmp/musl-link/target/debug/deps -L dependency=/tmp/musl-link/target/debug/deps --cap-lints allow`
INFO rustc_codegen_ssa::back::link preparing rlib to "/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib"
   Compiling foo v0.1.0 (/tmp/musl-link)
     Running `rustc --crate-name build_script_build --edition=2018 build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=b003b2bfd5b10605 -C extra-filename=-b003b2bfd5b10605 --out-dir /tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605 -C incremental=/tmp/musl-link/target/debug/incremental -L dependency=/tmp/musl-link/target/debug/deps --extern cc=/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib`
INFO rustc_codegen_ssa::back::link preparing Executable to "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605"
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.16li1oa9m4lbnp96.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.1qx5k5oex7x6tqmo.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2jetnrd27mc3ltzm.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2ow7cv6nritkbgw3.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2pm0l6oyqoxgntpi.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.3sp04hlw4rg70gs4.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4axkkc088djayh8s.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4l004g7h6c19xcvz.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4zwkuts8yv71i84y.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.5axvbt3rzlqvrln9.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.blncr3t7bimyyps.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.i43mgyrxk8qp10s.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.zv30i119hkpyip.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.3yqsbkxebwnqky0t.rcgu.o" "-Wl,--as-needed" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib" "-Wl,--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "-Wl,--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-nostartfiles" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605" "-Wl,--gc-sections" "-static-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
INFO rustc_codegen_ssa::back::link linker stderr:

INFO rustc_codegen_ssa::back::link linker stdout:

     Running `/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build-script-build`
     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=cb4158256382c543 -C extra-filename=-cb4158256382c543 --out-dir /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C incremental=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/tmp/musl-link/target/debug/deps -Ctarget-feature=+crt-static -Clinker=ld.bfd -Clink-arg=--verbose -L native=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out`
INFO rustc_codegen_ssa::back::link preparing rlib to "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib"
     Running `rustc --crate-name foo --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=656e6e56d7971844 -C extra-filename=-656e6e56d7971844 --out-dir /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C incremental=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/tmp/musl-link/target/debug/deps --extern foo=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib -Ctarget-feature=+crt-static -Clinker=ld.bfd -Clink-arg=--verbose -L native=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out`
INFO rustc_codegen_ssa::back::link preparing Executable to "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844"
INFO rustc_codegen_ssa::back::link "ld.bfd" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o" "--as-needed" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Bstatic" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib" "--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-lfoo" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844" "--gc-sections" "-static" "-pie" "--no-dynamic-linker" "-z" "text" "-zrelro" "-znow" "--verbose" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
error: linking with `ld.bfd` failed: exit status: 1
  |
  = note: "ld.bfd" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o" "--as-needed" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Bstatic" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib" "--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-lfoo" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844" "--gc-sections" "-static" "-pie" "--no-dynamic-linker" "-z" "text" "-zrelro" "-znow" "--verbose" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
  = note: ld.bfd: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a(foo.o): in function `bar':
          /tmp/musl-link/foo.c:3: undefined reference to `printf'
          ld.bfd: link errors found, deleting executable `/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844'
          GNU ld (Gentoo 2.36.1 p3) 2.36.1
            Supported emulations:
             elf_x86_64
             elf32_x86_64
             elf_i386
             elf_iamcu
             elf_l1om
             elf_k1om
          using internal linker script:
          ==================================================
          /* Script for -pie -z combreloc -z separate-code -z relro -z now */
          /* Copyright (C) 2014-2021 Free Software Foundation, Inc.
             Copying and distribution of this script, with or without modification,
             are permitted in any medium without royalty provided the copyright
             notice and this notice are preserved.  */
          OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64",
                      "elf64-x86-64")
          OUTPUT_ARCH(i386:x86-64)
          ENTRY(_start)
          SEARCH_DIR("/usr/x86_64-gentoo-linux-musl/lib64"); SEARCH_DIR("/usr/lib/binutils/x86_64-gentoo-linux-musl/2.36.164"); SEARCH_DIR("/usr/local/lib64"); SEARCH_DIR("/lib64"); SEARCH_DIR("/usr/lib64"); SEARCH_DIR("/usr/x86_64-gentoo-linux-musl/lib"); SEARCH_DIR("/usr/lib/binutils/x86_64-gentoo-linux-musl/2.36.1"); SEARCH_DIR("/usr/local/lib"); SEARCH_DIR("/lib"); SEARCH_DIR("/usr/lib");
          SECTIONS
          {
            PROVIDE (__executable_start = SEGMENT_START("text-segment", 0)); . = SEGMENT_START("text-segment", 0) + SIZEOF_HEADERS;
            .interp         : { *(.interp) }
            .note.gnu.build-id  : { *(.note.gnu.build-id) }
            .hash           : { *(.hash) }
            .gnu.hash       : { *(.gnu.hash) }
            .dynsym         : { *(.dynsym) }
            .dynstr         : { *(.dynstr) }
            .gnu.version    : { *(.gnu.version) }
            .gnu.version_d  : { *(.gnu.version_d) }
            .gnu.version_r  : { *(.gnu.version_r) }
            .rela.dyn       :
              {
                *(.rela.init)
                *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)
                *(.rela.fini)
                *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)
                *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)
                *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)
                *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)
                *(.rela.ctors)
                *(.rela.dtors)
                *(.rela.got)
                *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)
                *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)
                *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)
                *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)
                *(.rela.ifunc)
              }
            .rela.plt       :
              {
                *(.rela.plt)
                *(.rela.iplt)
              }
            . = ALIGN(CONSTANT (MAXPAGESIZE));
            .init           :
            {
              KEEP (*(SORT_NONE(.init)))
            }
            .plt            : { *(.plt) *(.iplt) }
          .plt.got        : { *(.plt.got) }
          .plt.sec        : { *(.plt.sec) }
            .text           :
            {
              *(.text.unlikely .text.*_unlikely .text.unlikely.*)
              *(.text.exit .text.exit.*)
              *(.text.startup .text.startup.*)
              *(.text.hot .text.hot.*)
              *(SORT(.text.sorted.*))
              *(.text .stub .text.* .gnu.linkonce.t.*)
              /* .gnu.warning sections are handled specially by elf.em.  */
              *(.gnu.warning)
            }
            .fini           :
            {
              KEEP (*(SORT_NONE(.fini)))
            }
            PROVIDE (__etext = .);
            PROVIDE (_etext = .);
            PROVIDE (etext = .);
            . = ALIGN(CONSTANT (MAXPAGESIZE));
            /* Adjust the address for the rodata segment.  We want to adjust up to
               the same address within the page on the next page up.  */
            . = SEGMENT_START("rodata-segment", ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)));
            .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }
            .rodata1        : { *(.rodata1) }
            .eh_frame_hdr   : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }
            .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }
            .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) }
            .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }
            /* These sections are generated by the Sun/Oracle C++ compiler.  */
            .exception_ranges   : ONLY_IF_RO { *(.exception_ranges*) }
            /* Adjust the address for the data segment.  We want to adjust up to
               the same address within the page on the next page up.  */
            . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));
            /* Exception handling  */
            .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }
            .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }
            .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }
            .exception_ranges   : ONLY_IF_RW { *(.exception_ranges*) }
            /* Thread Local Storage sections  */
            .tdata        :
             {
               PROVIDE_HIDDEN (__tdata_start = .);
               *(.tdata .tdata.* .gnu.linkonce.td.*)
             }
            .tbss                 : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }
            .preinit_array    :
            {
              PROVIDE_HIDDEN (__preinit_array_start = .);
              KEEP (*(.preinit_array))
              PROVIDE_HIDDEN (__preinit_array_end = .);
            }
            .init_array    :
            {
              PROVIDE_HIDDEN (__init_array_start = .);
              KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
              KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
              PROVIDE_HIDDEN (__init_array_end = .);
            }
            .fini_array    :
            {
              PROVIDE_HIDDEN (__fini_array_start = .);
              KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
              KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
              PROVIDE_HIDDEN (__fini_array_end = .);
            }
            .ctors          :
            {
              /* gcc uses crtbegin.o to find the start of
                 the constructors, so we make sure it is
                 first.  Because this is a wildcard, it
                 doesn't matter if the user does not
                 actually link against crtbegin.o; the
                 linker won't look for a file to match a
                 wildcard.  The wildcard also means that it
                 doesn't matter which directory crtbegin.o
                 is in.  */
              KEEP (*crtbegin.o(.ctors))
              KEEP (*crtbegin?.o(.ctors))
              /* We don't want to include the .ctor section from
                 the crtend.o file until after the sorted ctors.
                 The .ctor section from the crtend file contains the
                 end of ctors marker and it must be last */
              KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
              KEEP (*(SORT(.ctors.*)))
              KEEP (*(.ctors))
            }
            .dtors          :
            {
              KEEP (*crtbegin.o(.dtors))
              KEEP (*crtbegin?.o(.dtors))
              KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
              KEEP (*(SORT(.dtors.*)))
              KEEP (*(.dtors))
            }
            .jcr            : { KEEP (*(.jcr)) }
            .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }
            .dynamic        : { *(.dynamic) }
            .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }
            . = DATA_SEGMENT_RELRO_END (0, .);
            .data           :
            {
              *(.data .data.* .gnu.linkonce.d.*)
              SORT(CONSTRUCTORS)
            }
            .data1          : { *(.data1) }
            _edata = .; PROVIDE (edata = .);
            . = .;
            __bss_start = .;
            .bss            :
            {
             *(.dynbss)
             *(.bss .bss.* .gnu.linkonce.b.*)
             *(COMMON)
             /* Align here to ensure that the .bss section occupies space up to
                _end.  Align after .bss to ensure correct alignment even if the
                .bss section disappears because there are no input sections.
                FIXME: Why do we need it? When there is no .bss section, we do not
                pad the .data section.  */
             . = ALIGN(. != 0 ? 64 / 8 : 1);
            }
            .lbss   :
            {
              *(.dynlbss)
              *(.lbss .lbss.* .gnu.linkonce.lb.*)
              *(LARGE_COMMON)
            }
            . = ALIGN(64 / 8);
            . = SEGMENT_START("ldata-segment", .);
            .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
            {
              *(.lrodata .lrodata.* .gnu.linkonce.lr.*)
            }
            .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
            {
              *(.ldata .ldata.* .gnu.linkonce.l.*)
              . = ALIGN(. != 0 ? 64 / 8 : 1);
            }
            . = ALIGN(64 / 8);
            _end = .; PROVIDE (end = .);
            . = DATA_SEGMENT_END (.);
            /* Stabs debugging sections.  */
            .stab          0 : { *(.stab) }
            .stabstr       0 : { *(.stabstr) }
            .stab.excl     0 : { *(.stab.excl) }
            .stab.exclstr  0 : { *(.stab.exclstr) }
            .stab.index    0 : { *(.stab.index) }
            .stab.indexstr 0 : { *(.stab.indexstr) }
            .comment       0 : { *(.comment) }
            .gnu.build.attributes : { *(.gnu.build.attributes .gnu.build.attributes.*) }
            /* DWARF debug sections.
               Symbols in the DWARF debugging sections are relative to the beginning
               of the section so we begin them at 0.  */
            /* DWARF 1.  */
            .debug          0 : { *(.debug) }
            .line           0 : { *(.line) }
            /* GNU DWARF 1 extensions.  */
            .debug_srcinfo  0 : { *(.debug_srcinfo) }
            .debug_sfnames  0 : { *(.debug_sfnames) }
            /* DWARF 1.1 and DWARF 2.  */
            .debug_aranges  0 : { *(.debug_aranges) }
            .debug_pubnames 0 : { *(.debug_pubnames) }
            /* DWARF 2.  */
            .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
            .debug_abbrev   0 : { *(.debug_abbrev) }
            .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }
            .debug_frame    0 : { *(.debug_frame) }
            .debug_str      0 : { *(.debug_str) }
            .debug_loc      0 : { *(.debug_loc) }
            .debug_macinfo  0 : { *(.debug_macinfo) }
            /* SGI/MIPS DWARF 2 extensions.  */
            .debug_weaknames 0 : { *(.debug_weaknames) }
            .debug_funcnames 0 : { *(.debug_funcnames) }
            .debug_typenames 0 : { *(.debug_typenames) }
            .debug_varnames  0 : { *(.debug_varnames) }
            /* DWARF 3.  */
            .debug_pubtypes 0 : { *(.debug_pubtypes) }
            .debug_ranges   0 : { *(.debug_ranges) }
            /* DWARF 5.  */
            .debug_addr     0 : { *(.debug_addr) }
            .debug_line_str 0 : { *(.debug_line_str) }
            .debug_loclists 0 : { *(.debug_loclists) }
            .debug_macro    0 : { *(.debug_macro) }
            .debug_names    0 : { *(.debug_names) }
            .debug_rnglists 0 : { *(.debug_rnglists) }
            .debug_str_offsets 0 : { *(.debug_str_offsets) }
            .debug_sup      0 : { *(.debug_sup) }
            .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }
            /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
          }
          
          
          ==================================================
          ld.bfd: mode elf_x86_64
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib
          (/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib)foo-cb4158256382c543.1lnt8gom92t6x66c.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib)std-f1cbe3fb941687fb.std.3ce96de4-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib)panic_unwind-6774d3dce98216e2.panic_unwind.3d814f7e-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib)miniz_oxide-2b70a69428b67501.miniz_oxide.9ff05367-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib)adler-7c0d8eb24f55575d.adler.750c459d-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib)object-39efbcf3ca573d24.object.b115ab7c-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib)addr2line-65df03b5b3a62814.addr2line.b058eadf-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib)gimli-6c5352065190f68a.gimli.19381030-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib)rustc_demangle-255ccea4ef40f4d4.rustc_demangle.eccc3d78-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib)UnwindLevel1-gcc-ext.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib)UnwindLevel1.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib)UnwindRegistersSave.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib)libunwind.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib)UnwindRegistersRestore.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sysconf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)closedir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dirfd.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)opendir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)readdir_r.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__environ.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__libc_start_main.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getenv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setenv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)unsetenv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__errno_location.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)abort.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)assert.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)exit.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fcntl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)open.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)defsysinfo.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)libc.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)syscall_ret.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dl_iterate_phdr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dladdr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dlsym.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)chroot.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)prctl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sendfile.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setgroups.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sysinfo.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lite_malloc.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)malloc.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_memalign.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fma.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fmaf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getrlimit.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)ioctl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)realpath.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)syscall.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)madvise.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mmap.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mprotect.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mremap.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)munmap.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)accept4.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)bind.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)connect.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)freeaddrinfo.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)gai_strerror.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getaddrinfo.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getpeername.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getsockname.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getsockopt.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)htons.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)listen.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lookup_name.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lookup_serv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)recv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)recvfrom.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)recvmsg.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)res_mkquery.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)res_msend.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)resolvconf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)send.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sendmsg.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sendto.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setsockopt.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)shutdown.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)socket.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)socketpair.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getpw_r.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)execvp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fork.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawn_file_actions_adddup2.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawn_file_actions_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawn_file_actions_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnattr_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnattr_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnattr_setflags.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnattr_setsigdefault.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnattr_setsigmask.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawnp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)waitpid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sched_yield.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)poll.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)block.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)kill.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)raise.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sigaction.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sigaddset.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sigaltstack.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sigemptyset.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)signal.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)restore.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)chmod.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fchmod.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fstat.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fstatat.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lstat.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mkdir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)stat.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__fclose_ca.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__fopen_rb_ca.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__stdio_close.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__stdio_read.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__stdio_seek.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fflush.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fgets.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fprintf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fwrite.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getc.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)ofl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)rename.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)snprintf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)stderr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)vfprintf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)vsnprintf.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)qsort.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strtol.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)bcmp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memchr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memcmp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memrchr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strchr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strchrnul.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strcpy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strdup.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strerror_r.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strlen.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strncmp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strnlen.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strstr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memcpy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memmove.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memset.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__lock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__syscall_cp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__tls_get_addr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__wait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_attr_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_attr_get.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_attr_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_attr_setstacksize.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cleanup_push.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_broadcast.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_signal.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_timedwait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_cond_wait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_condattr_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_condattr_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_condattr_setclock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_create.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_detach.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_getattr_np.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_getspecific.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_join.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_key_create.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_lock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_timedlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_trylock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutex_unlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutexattr_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutexattr_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_mutexattr_settype.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_rdlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_timedrdlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_tryrdlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_unlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_wrlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_self.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_setcancelstate.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_setspecific.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_sigmask.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_testcancel.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)vmlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__unmapself.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)clone.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)clock_gettime.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)nanosleep.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)_exit.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)chdir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)close.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dup2.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fdatasync.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fsync.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)ftruncate.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getcwd.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getpid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getppid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getuid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)linkat.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lseek.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pipe2.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pread.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pwrite.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)read.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)readlink.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)readv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)rmdir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setgid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setuid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)setxid.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)symlink.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)unlink.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)write.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)writev.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)isalnum.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)readdir.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__init_tls.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)putenv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strerror.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)_Exit.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fesetround.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fenv.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)intscan.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)procfdname.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)shgetc.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)vdso.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__dlsym.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dlerror.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__lctrans.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)expand_heap.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)memalign.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__fpclassifyl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__signbitl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)frexpl.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)scalbn.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mbstowcs.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)wctomb.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)accept.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dn_expand.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)dns_parse.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)lookup_ipliteral.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getpw_a.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getpwent_a.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)nscd_query.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)execve.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)posix_spawn.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sigismember.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__fdopen.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__lockfile.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__stdio_write.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__towrite.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__uflow.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fclose.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)ferror.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fopen.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)fread.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getline.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)ofl_add.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)stpcpy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strcmp.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__timedwait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)default_attr.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_timedwrlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pthread_rwlock_trywrlock.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)synccall.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__set_thread_area.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)clock_nanosleep.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)pipe.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)mbsrtowcs.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)wcrtomb.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)if_nametoindex.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)inet_aton.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)inet_pton.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__fmodeflags.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__stdio_exit.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)__toread.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)getdelim.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)strncpy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_destroy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_init.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_post.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_wait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)internal.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)stpncpy.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_timedwait.lo
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib)sem_trywait.lo
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib)alloc-b292e64c32db2ac6.alloc.b9848c4b-cgu.0.rcgu.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib)core-58e3a51e774ec0f7.core.52ce99a3-cgu.0.rcgu.o
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib)compiler_builtins-5added8ca6387776.compiler_builtins.a736959a-cgu.35.rcgu.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib)compiler_builtins-5added8ca6387776.compiler_builtins.a736959a-cgu.4.rcgu.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib)compiler_builtins-5added8ca6387776.compiler_builtins.a736959a-cgu.40.rcgu.o
          (/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib)compiler_builtins-5added8ca6387776.compiler_builtins.a736959a-cgu.83.rcgu.o
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo.a failed
          attempt to open /tmp/musl-link/target/debug/deps/libfoo.a failed
          attempt to open /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a succeeded
          /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a
          (/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a)foo.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o
          attempt to open /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o succeeded
          /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `foo` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name foo --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=656e6e56d7971844 -C extra-filename=-656e6e56d7971844 --out-dir /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C incremental=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/tmp/musl-link/target/debug/deps --extern foo=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib -Ctarget-feature=+crt-static -Clinker=ld.bfd -Clink-arg=--verbose -L native=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out` (exit status: 1)
```

# Ok


```shell
export RUSTC_LOG=rustc_codegen_ssa::back::link=info
export RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=ld.lld -Clink-arg=--verbose"
cargo run --target=x86_64-unknown-linux-musl -v
```

```
  Compiling cc v1.0.69
     Running `rustc --crate-name cc --edition=2018 /home/han/.cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.69/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1d671f13a31d6bec -C extra-filename=-1d671f13a31d6bec --out-dir /tmp/musl-link/target/debug/deps -L dependency=/tmp/musl-link/target/debug/deps --cap-lints allow`
INFO rustc_codegen_ssa::back::link preparing rlib to "/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib"
   Compiling foo v0.1.0 (/tmp/musl-link)
     Running `rustc --crate-name build_script_build --edition=2018 build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=b003b2bfd5b10605 -C extra-filename=-b003b2bfd5b10605 --out-dir /tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605 -C incremental=/tmp/musl-link/target/debug/incremental -L dependency=/tmp/musl-link/target/debug/deps --extern cc=/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib`
INFO rustc_codegen_ssa::back::link preparing Executable to "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605"
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.16li1oa9m4lbnp96.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.1qx5k5oex7x6tqmo.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2jetnrd27mc3ltzm.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2ow7cv6nritkbgw3.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.2pm0l6oyqoxgntpi.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.3sp04hlw4rg70gs4.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4axkkc088djayh8s.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4l004g7h6c19xcvz.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.4zwkuts8yv71i84y.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.5axvbt3rzlqvrln9.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.blncr3t7bimyyps.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.i43mgyrxk8qp10s.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.zv30i119hkpyip.rcgu.o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605.3yqsbkxebwnqky0t.rcgu.o" "-Wl,--as-needed" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "/tmp/musl-link/target/debug/deps/libcc-1d671f13a31d6bec.rlib" "-Wl,--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "-Wl,--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-nostartfiles" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build_script_build-b003b2bfd5b10605" "-Wl,--gc-sections" "-static-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
INFO rustc_codegen_ssa::back::link linker stderr:

INFO rustc_codegen_ssa::back::link linker stdout:

     Running `/tmp/musl-link/target/debug/build/foo-b003b2bfd5b10605/build-script-build`
     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=cb4158256382c543 -C extra-filename=-cb4158256382c543 --out-dir /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C incremental=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/tmp/musl-link/target/debug/deps -Ctarget-feature=+crt-static -Clinker=ld.lld -Clink-arg=--verbose -L native=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out`
INFO rustc_codegen_ssa::back::link preparing rlib to "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib"
     Running `rustc --crate-name foo --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=656e6e56d7971844 -C extra-filename=-656e6e56d7971844 --out-dir /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C incremental=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/tmp/musl-link/target/debug/deps --extern foo=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib -Ctarget-feature=+crt-static -Clinker=ld.lld -Clink-arg=--verbose -L native=/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out`
INFO rustc_codegen_ssa::back::link preparing Executable to "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844"
INFO rustc_codegen_ssa::back::link "ld.lld" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o" "--as-needed" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/tmp/musl-link/target/debug/deps" "-L" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Bstatic" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib" "--start-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib" "--end-group" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib" "-lfoo" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844" "--gc-sections" "-static" "-pie" "--no-dynamic-linker" "-z" "text" "-zrelro" "-znow" "--verbose" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
INFO rustc_codegen_ssa::back::link linker stderr:
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.15pu1hl0kal84f1y.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1h68etnkmxrn6yzz.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1pe73trdgmny6a8t.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.1zms9s2k5rtx003u.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.2re1n6nelkxha2og.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.36sds9xqyeuceja1.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.3nlkyw0e8uutnyey.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.8q3ae5zw7l4m4j0.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.i3svhsqj32n5bqa.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.j7ech079p6rmvfx.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.wktolicz9f1e7px.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/foo-656e6e56d7971844.46l3v0trxis9yeee.rcgu.o
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/deps/libfoo-cb4158256382c543.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-f1cbe3fb941687fb.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-6774d3dce98216e2.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-2b70a69428b67501.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-7c0d8eb24f55575d.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-39efbcf3ca573d24.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-65df03b5b3a62814.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-6c5352065190f68a.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-7a40648f799302f6.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-255ccea4ef40f4d4.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-e6cbd0b0da1406a9.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-6be43c7f9d293971.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-a8a6eb096906f234.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-84f9c9d47d8869d8.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-1d56f1e04d9d51b1.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-b292e64c32db2ac6.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d144bec3bbe14c26.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-58e3a51e774ec0f7.rlib
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5added8ca6387776.rlib
ld.lld: /tmp/musl-link/target/x86_64-unknown-linux-musl/debug/build/foo-c6c7efd0b76b3015/out/libfoo.a
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o
ld.lld: /home/han/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o

INFO rustc_codegen_ssa::back::link linker stdout:

    Finished dev [unoptimized + debuginfo] target(s) in 2.45s
     Running `target/x86_64-unknown-linux-musl/debug/foo`
foo.c here!
foo.c return 42!
```