fn init(script_id, script_store, global_store) {
	script_store.set("counter", 0);

	let import_slots = get_import_slots();

	for slot in import_slots {
		if slot.module_name == "<main>" && slot.import_name == "printf" {
			attach_hook(slot, HookPhase::Prefix, HookStatus::Active, HookPriority(1), [
				Parameter::create(
					ParameterLocation::Reg(Register::Rdi),
					ParameterValue::Ptr([
						Parameter::create(
							ParameterLocation::Relative(0),
							ParameterValue::CStr,
						)
					])
				)
			], script_id, |script_id, script_store, global_store, parameters| {
				let count = script_store.get("counter") + 1;
				script_store.set("counter", count);
			
				print(`printf hook hit ${count} time(s)`);

				print(parameters);

				parameters
			});
		}
	}
}
