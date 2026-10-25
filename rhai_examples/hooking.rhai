fn init(script_id, script_store, global_store) {
	script_store.set("counter", 0);
	script_store.set("hooks_enabled", true);

	let import_slots = get_import_slots();

	let rax_accessor = Parameter::create(
		ParameterLocation::Reg(Register::Rax),
		ParameterValue::U64,
	);

	let format_string_accessor = Parameter::create(
		ParameterLocation::Reg(Register::Rdi),
		ParameterValue::Ptr([
			Parameter::create(
				ParameterLocation::Relative(0),
				ParameterValue::CStr,
			)
		])
	);

	for slot in import_slots {
		if slot.module_name == "<main>" && slot.import_name == "printf" {
			attach_hook(
				slot,
				HookPhase::Prefix,
				HookStatus::Active,
				HookOrder(0),
				[
					rax_accessor,
					format_string_accessor,
				],
				script_id,
				|script_id, script_store, global_store, parameters, call_ctrl| {
					if !script_store.get("hooks_enabled") {
						return [];
					}

					let count = script_store.get("counter") + 1;
					script_store.set("counter", count);
			
					print(`printf hook hit ${count} time(s)`);

					print(parameters);

					[]
				}
			);
		}

		if slot.module_name == "<main>" && slot.import_name == "getpid" {
			attach_hook(
				slot,
				HookPhase::Postfix,
				HookStatus::Active,
				HookOrder(0),
				[
					rax_accessor,
				],
				script_id,
				|script_id, script_store, global_store, parameters| {
					if !script_store.get("hooks_enabled") {
						return [];
					}

					parameters[0].value = ParameterValue::U64(69420);

					parameters
				}
			);
		}
	}
}

fn toggle_hooks(script_id, script_store, global_store) {
	script_store.set("hooks_enabled", !script_store.get("hooks_enabled"));
}
