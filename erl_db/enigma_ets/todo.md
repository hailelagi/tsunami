how to test this?

enigma relied on including upstream erlang/otp as a git submodule and BIFs evaluated when the compiler inlined:

```
#[inline]
pub fn apply(vm: &Machine, process: &RcProcess, mfa: &module::MFA, args: &[Term]) -> Result {
    // println!("bif_apply {}", mfa);
    match BIFS.get(mfa) {
        Some(fun) => fun(vm, process, args),
        None => unimplemented!("BIF {} not implemented", mfa),
    }
}
```

from a pub static BIFS: Lazy<BifTable>:

``
       "ets" => {
            "new", 2 => ets::bif::new_2,
            "whereis", 1 => ets::bif::whereis_1,
            "insert", 2 => ets::bif::insert_2,
            "insert_new", 2 => ets::bif::insert_new_2,
            "lookup", 2 => ets::bif::lookup_2,
            "lookup_element", 3 => ets::bif::lookup_element_3,
            "delete", 1 => ets::bif::delete_1,
            "delete", 2 => ets::bif::delete_2,
            "select", 2 => ets::bif::select_2,
            "select_delete", 2 => ets::bif::select_delete_2,
            "update_element", 3 => ets::bif::update_element_3,
            "match", 2 => ets::bif::match_2,
            "member", 2 => ets::bif::member_2,
            "first", 1 => ets::bif::first_1,
            "last", 1 => ets::bif::last_1,
            "info", 2 => ets::bif::info_2,
        },
``

this presents multiple problems for decoupling test as it depends on tight integration with not only the VM,
but also the compiler.

