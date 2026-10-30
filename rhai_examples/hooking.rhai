fn init(script_id, script_store, global_store) {
	script_store.set("counter", 0);
	script_store.set("hooks_enabled", true);

	let rax_accessor = Parameter::create(
		ParameterLocation::Reg(Register::Rax),
		ParameterValue::U64,
	);

	let format_string_accessor = Parameter::deref(
		ParameterLocation::Reg(Register::Rdi),
		ParameterValue::CStr,
	);
	
	// Expands to:
	// let format_string_accessor = Parameter::create(
	// 	ParameterLocation::Reg(Register::Rdi),
	// 	ParameterValue::Ptr([
	// 		Parameter::create(
	// 			ParameterLocation::Relative(0),
	// 			ParameterValue::CStr,
	// 		)
	// 	])
	// );

	let printf_slots = find_import_slots("<main>", "printf");

	for slot in printf_slots {
		attach_hook(
			slot,
			HookPhase::Prefix,
			HookStatus::Active,
			HookOrder(0),
			#{
				format_string: format_string_accessor,
			},
			script_id,
			|script_id, script_store, global_store, parameters, call_ctrl| {
				if !script_store.get("hooks_enabled") {
					return [];
				}

				let count = script_store.get("counter") + 1;
				script_store.set("counter", count);

				// Prefix functions can prevent the original from being ran
				// call_ctrl.cancel();
				
				print(`printf hook hit ${count} time(s)`);

				print(`Format string: ${parameters.format_string.val}`);

				parameters
			}
		);
	}

	let getpid_slots = find_import_slots("<main>", "getpid");
	
	for slot in getpid_slots {
		attach_hook(
			slot,
			HookPhase::Postfix,
			HookStatus::Active,
			HookOrder(0),
			#{
				ret: rax_accessor,
			},
			script_id,
			|script_id, script_store, global_store, parameters| {
				if !script_store.get("hooks_enabled") {
					return [];
				}

				parameters.ret.val = ParameterValue::U64(69420);

				parameters
			}
		);
	}
}

fn toggle_hooks(script_id, script_store, global_store) {
	script_store.set("hooks_enabled", !script_store.get("hooks_enabled"));
}
