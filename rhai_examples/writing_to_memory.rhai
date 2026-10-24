fn init(script_id, script_store, global_store) {
	debug(script_id);

	// Edit the address to match the output string pointer
	let address = 0x5599d4d9d470;
	
	mprotect(address, 12, PROT_WRITE | PROT_READ);
	write_mem(address, "Hacked");
}
